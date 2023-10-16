#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw721::ContractInfoResponse;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, RewardExecuteMsg, RewardQueryMsg};
use crate::state::{Config, Metadata, CONFIG};

const PUNKGA_REWARD_NAME: &str = "punkga-reward";
const PUNKGA_REWARD_SYMBOL: &str = "PGR";

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:punkga-reward-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type RewardNftContract<'a> =
    cw721_base::Cw721Contract<'a, Metadata, Empty, RewardExecuteMsg, RewardQueryMsg>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // save contract config
    let config = Config {
        admin: deps.api.addr_validate(&msg.admin)?,
    };
    CONFIG.save(deps.storage, &config)?;

    let reward_nft = RewardNftContract::default();

    // Save contract info
    let info = ContractInfoResponse {
        name: PUNKGA_REWARD_NAME.to_owned(),
        symbol: PUNKGA_REWARD_SYMBOL.to_owned(),
    };
    reward_nft.contract_info.save(deps.storage, &info)?;

    // initialize owner
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.minter))?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", msg.admin.to_string())
        .add_attribute("minter", msg.minter.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let reward_nft = RewardNftContract::default();
    reward_nft.execute(deps, env, info, msg).map_err(Into::into)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let reward_nft = RewardNftContract::default();
    reward_nft.query(deps, env, msg)
}

#[cfg(test)]
mod tests {}
