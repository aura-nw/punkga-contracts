use cosmwasm_schema::{cw_serde, QueryResponses};
use cw2981_royalties::Metadata;

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub reward_code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        admin: String,
        reward_code_id: u64,
    },
    MintReward {
        user_addr: String,
        token_id: String,
        token_uri: Option<String>,
        extension: Option<Metadata>,
    },
    UpdateUserInfo {
        address: String,
        level: u64,
        total_xp: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
}
