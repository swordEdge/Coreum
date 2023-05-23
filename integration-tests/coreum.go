package integrationtests

import (
	"context"
	"fmt"
	"reflect"

	cosmosed25519 "github.com/cosmos/cosmos-sdk/crypto/keys/ed25519"
	sdk "github.com/cosmos/cosmos-sdk/types"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
	"github.com/pkg/errors"
	"go.uber.org/zap"

	"github.com/CoreumFoundation/coreum-tools/pkg/logger"
	"github.com/CoreumFoundation/coreum/pkg/client"
	"github.com/CoreumFoundation/coreum/x/deterministicgas"
)

// CoreumChain is configured coreum chain.
type CoreumChain struct {
	Chain
	Governance             Governance
	DeterministicGasConfig deterministicgas.Config
}

// NewCoreumChain returns a new instance of the CoreumChain.
func NewCoreumChain(chain Chain, stakerMnemonics []string) CoreumChain {
	return CoreumChain{
		Chain:                  chain,
		Governance:             NewGovernance(chain.ChainContext, stakerMnemonics, chain.Faucet),
		DeterministicGasConfig: deterministicgas.DefaultConfig(),
	}
}

// BalancesOptions is the input type for the ComputeNeededBalanceFromOptions.
type BalancesOptions struct {
	Messages                    []sdk.Msg
	NondeterministicMessagesGas uint64
	GasPrice                    sdk.Dec
	Amount                      sdk.Int
}

// GasLimitByMsgs calculates sum of gas limits required for message types passed.
// It panics if unsupported message type specified.
func (c CoreumChain) GasLimitByMsgs(msgs ...sdk.Msg) uint64 {
	var totalGasRequired uint64
	for _, msg := range msgs {
		msgGas, exists := c.DeterministicGasConfig.GasRequiredByMessage(msg)
		if !exists {
			panic(errors.Errorf("unsuported message type for deterministic gas: %v", reflect.TypeOf(msg).String()))
		}
		totalGasRequired += msgGas + c.DeterministicGasConfig.FixedGas
	}

	return totalGasRequired
}

// GasLimitByMultiSendMsgs calculates sum of gas limits required for message types passed and includes the FixedGas once.
// It panics if unsupported message type specified.
func (c CoreumChain) GasLimitByMultiSendMsgs(msgs ...sdk.Msg) uint64 {
	var totalGasRequired uint64
	for _, msg := range msgs {
		msgGas, exists := c.DeterministicGasConfig.GasRequiredByMessage(msg)
		if !exists {
			panic(errors.Errorf("unsuported message type for deterministic gas: %v", reflect.TypeOf(msg).String()))
		}
		totalGasRequired += msgGas
	}

	return totalGasRequired + c.DeterministicGasConfig.FixedGas
}

// ComputeNeededBalanceFromOptions computes the required balance based on the input options.
func (c CoreumChain) ComputeNeededBalanceFromOptions(options BalancesOptions) sdk.Int {
	if options.GasPrice.IsNil() {
		options.GasPrice = c.ChainSettings.GasPrice
	}

	if options.Amount.IsNil() {
		options.Amount = sdk.ZeroInt()
	}

	// NOTE: we assume that each message goes to one transaction, which is not
	// very accurate and may cause some over funding in cases that there are multiple
	// messages in a single transaction
	totalAmount := sdk.ZeroInt()
	for _, msg := range options.Messages {
		gas := c.GasLimitByMsgs(msg)
		// Ceil().RoundInt() is here to be compatible with the sdk's TxFactory
		// https://github.com/cosmos/cosmos-sdk/blob/ff416ee63d32da5d520a8b2d16b00da762416146/client/tx/factory.go#L223
		amt := options.GasPrice.Mul(sdk.NewDec(int64(gas))).Ceil().RoundInt()
		totalAmount = totalAmount.Add(amt)
	}

	return totalAmount.Add(options.GasPrice.Mul(sdk.NewDec(int64(options.NondeterministicMessagesGas))).Ceil().RoundInt()).Add(options.Amount)
}

// FundAccountsWithOptions computes the needed balances and fund account with it.
func (c CoreumChain) FundAccountsWithOptions(ctx context.Context, address sdk.AccAddress, options BalancesOptions) error {
	amount := c.ComputeNeededBalanceFromOptions(options)
	return c.Faucet.FundAccounts(ctx, FundedAccount{
		Address: address,
		Amount:  c.NewCoin(amount),
	})
}

// CreateValidator creates a new validator on the chain and returns the staker addresses, validator addresses and callback function to deactivate it.
func (c CoreumChain) CreateValidator(ctx context.Context, stakingAmount, selfDelegationAmount sdk.Int) (sdk.AccAddress, sdk.ValAddress, func() error, error) {
	stakingClient := stakingtypes.NewQueryClient(c.ClientContext)
	staker := c.GenAccount()

	if err := c.FundAccountsWithOptions(ctx, staker, BalancesOptions{
		Messages: []sdk.Msg{&stakingtypes.MsgCreateValidator{}, &stakingtypes.MsgUndelegate{}},
		Amount:   stakingAmount,
	}); err != nil {
		return nil, nil, nil, err
	}

	// Create staker
	validatorAddr := sdk.ValAddress(staker)
	msg, err := stakingtypes.NewMsgCreateValidator(
		validatorAddr,
		cosmosed25519.GenPrivKey().PubKey(),
		c.NewCoin(stakingAmount),
		stakingtypes.Description{Moniker: fmt.Sprintf("testing-staker-%s", staker)},
		stakingtypes.NewCommissionRates(sdk.MustNewDecFromStr("0.1"), sdk.MustNewDecFromStr("0.1"), sdk.MustNewDecFromStr("0.1")),
		selfDelegationAmount,
	)
	if err != nil {
		return nil, nil, nil, err
	}

	result, err := client.BroadcastTx(
		ctx,
		c.ClientContext.WithFromAddress(staker),
		c.TxFactory().WithGas(c.GasLimitByMsgs(msg)),
		msg,
	)
	if err != nil {
		return nil, nil, nil, err
	}

	logger.Get(ctx).Info("Validator creation executed", zap.String("txHash", result.TxHash))

	// Make sure staker has been created
	resp, err := stakingClient.Validator(ctx, &stakingtypes.QueryValidatorRequest{
		ValidatorAddr: validatorAddr.String(),
	})
	if err != nil {
		return nil, nil, nil, errors.WithStack(err)
	}
	if stakingAmount.String() != resp.Validator.Tokens.String() {
		return nil, nil, nil, errors.Errorf("unexpected validator %q tokens after creation: %s", validatorAddr, resp.Validator.Tokens)
	}
	if stakingtypes.Bonded != resp.Validator.Status {
		return nil, nil, nil, errors.Errorf("unexpected validator %q status after creation: %s", validatorAddr, resp.Validator.Status)
	}

	return staker, validatorAddr, func() error {
		// Undelegate coins, i.e. deactivate staker
		undelegateMsg := stakingtypes.NewMsgUndelegate(staker, validatorAddr, c.NewCoin(stakingAmount))
		_, err = client.BroadcastTx(
			ctx,
			c.ClientContext.WithFromAddress(staker),
			c.TxFactory().WithSimulateAndExecute(true),
			undelegateMsg,
		)
		if err != nil {
			return err
		}

		// make sure the validator isn't bonded now
		resp, err := stakingClient.Validator(ctx, &stakingtypes.QueryValidatorRequest{
			ValidatorAddr: validatorAddr.String(),
		})
		if err != nil {
			return errors.WithStack(err)
		}

		if stakingtypes.Bonded == resp.Validator.Status {
			return errors.Errorf("unexpected validator %q status after removal: %s", validatorAddr, resp.Validator.Status)
		}

		return nil
	}, nil
}