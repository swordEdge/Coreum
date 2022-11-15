use cosmwasm_std::{CosmosMsg, CustomMsg, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum FungibleTokenMsg {
    MsgIssueFungibleToken {
        symbol: String,
        recipient: String,
        initial_amount: Uint128,
    },
}

impl Into<CosmosMsg<FungibleTokenMsg>> for FungibleTokenMsg {
    fn into(self) -> CosmosMsg<FungibleTokenMsg> {
        CosmosMsg::Custom(self)
    }
}

impl CustomMsg for FungibleTokenMsg {}
