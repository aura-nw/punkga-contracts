use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub reward_code_id: u64,
}

#[cw_serde]
pub struct UserInfo {
    pub address: Addr,
    pub level: u64,
    pub total_xp: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const USER_INFOS: Map<&str, UserInfo> = Map::new("user_infos");
pub const REWARD_CONTRACT: Item<Addr> = Item::new("reward_contract");
