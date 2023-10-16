use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomMsg;

use crate::state::{Config, Metadata};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub minter: String,
}

pub type ExecuteMsg = cw721_base::ExecuteMsg<Metadata, RewardExecuteMsg>;
#[cw_serde]
pub enum RewardExecuteMsg {}

pub type QueryMsg = cw721_base::QueryMsg<RewardQueryMsg>;

#[cw_serde]
#[derive(QueryResponses)]
pub enum RewardQueryMsg {
    #[returns(Config)]
    Config {},
}

impl CustomMsg for RewardExecuteMsg {}
impl CustomMsg for RewardQueryMsg {}
