use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub nft_code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig { admin: String, nft_code_id: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
}
