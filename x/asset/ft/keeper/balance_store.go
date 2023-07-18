package keeper

import (
	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/cosmos/cosmos-sdk/types/query"

	"github.com/CoreumFoundation/coreum/v2/x/asset/ft/types"
)

func newBalanceStore(cdc codec.BinaryCodec, store sdk.KVStore, pref []byte) balanceStore {
	return balanceStore{
		cdc:   cdc,
		store: prefix.NewStore(store, pref),
	}
}

// balanceStore is the unified store for getting balance of an accounts, currently it is used by freezing and whitelisting.
type balanceStore struct {
	store prefix.Store
	cdc   codec.BinaryCodec
}

func (s balanceStore) Balance(denom string) sdk.Coin {
	balance := sdk.NewCoin(denom, sdk.ZeroInt())
	if bz := s.store.Get([]byte(denom)); bz != nil {
		s.cdc.MustUnmarshal(bz, &balance)
	}

	return balance
}

func (s balanceStore) Balances(pagination *query.PageRequest) (sdk.Coins, *query.PageResponse, error) {
	coinPointers, pageRes, err := query.GenericFilteredPaginate(
		s.cdc,
		s.store,
		pagination,
		// builder
		func(key []byte, coin *sdk.Coin) (*sdk.Coin, error) {
			return coin, nil
		},
		// constructor
		func() *sdk.Coin {
			return &sdk.Coin{}
		},
	)
	if err != nil {
		return nil, nil, sdkerrors.Wrapf(types.ErrInvalidInput, "failed to paginate: %s", err)
	}

	coins := make(sdk.Coins, 0, len(coinPointers))
	for _, c := range coinPointers {
		coins = append(coins, *c)
	}

	return coins, pageRes, nil
}

// IterateAllBalances iterates over all balances of all accounts and applies the provided callback.
// If true is returned from the callback, iteration is stopped.
func (s balanceStore) IterateAllBalances(cb func(sdk.AccAddress, sdk.Coin) bool) error {
	iterator := s.store.Iterator(nil, nil)
	defer iterator.Close()

	for ; iterator.Valid(); iterator.Next() {
		address, err := types.AddressFromBalancesStore(iterator.Key())
		if err != nil {
			return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid address in the balances store saved with key: %s", string(iterator.Key()))
		}

		var balance sdk.Coin
		s.cdc.MustUnmarshal(iterator.Value(), &balance)

		if cb(address, balance) {
			break
		}
	}

	return nil
}

func (s balanceStore) SetBalance(coin sdk.Coin) {
	if coin.Amount.IsZero() {
		s.store.Delete([]byte(coin.Denom))
	} else {
		bz := s.cdc.MustMarshal(&coin)
		s.store.Set([]byte(coin.Denom), bz)
	}
}

func collectBalances(cdc codec.BinaryCodec, store sdk.KVStore, pagination *query.PageRequest) ([]types.Balance, *query.PageResponse, error) {
	var balances []types.Balance
	mapAddressToBalancesIdx := make(map[string]int)
	pageRes, err := query.Paginate(store, pagination, func(key, value []byte) error {
		address, err := types.AddressFromBalancesStore(key)
		if err != nil {
			return err
		}

		var coin sdk.Coin
		cdc.MustUnmarshal(value, &coin)

		idx, ok := mapAddressToBalancesIdx[address.String()]
		if ok {
			// address is already on the set of accounts balances
			balances[idx].Coins = balances[idx].Coins.Add(coin)
			balances[idx].Coins.Sort()
			return nil
		}

		accountBalance := types.Balance{
			Address: address.String(),
			Coins:   sdk.NewCoins(coin),
		}
		balances = append(balances, accountBalance)
		mapAddressToBalancesIdx[address.String()] = len(balances) - 1
		return nil
	})

	return balances, pageRes, err
}
