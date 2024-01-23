use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response,
    StdError, StdResult, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw2981_royalties::Metadata;
use cw_utils::parse_reply_instantiate_data;
#[cfg(not(feature = "library"))]
use std::vec;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, UserInfo, CONFIG, REWARD_CONTRACT, USER_INFOS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:punkga-leveling";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // save contract config
    let admin = deps.api.addr_validate(&msg.admin)?;
    let config = Config {
        admin: admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;

    let reward_ins_msg = CosmosMsg::Wasm(WasmMsg::Instantiate {
        admin: Some(admin.to_string()),
        code_id: msg.reward_code_id,
        msg: to_json_binary(&cw2981_royalties::InstantiateMsg {
            minter: env.contract.address.to_string(),
            name: msg.reward_name.clone(),
            symbol: msg.reward_symbol.clone(),
        })?,
        funds: vec![],
        label: "punkga_reward_nft".to_owned(),
    });

    let reward_ins_submsg = SubMsg {
        id: REPLY_ID,
        msg: reward_ins_msg,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    Ok(Response::new()
        .add_submessage(reward_ins_submsg)
        .add_attribute("action", "instantiate")
        .add_attribute("admin", msg.admin.to_string())
        .add_attribute("reward_name", msg.reward_name)
        .add_attribute("reward_symbol", msg.reward_symbol.clone())
        .add_attribute("reward_code_id", msg.reward_code_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { admin } => execute_update_config(deps, env, info, admin),
        ExecuteMsg::MintReward {
            user_addr,
            token_id,
            token_uri,
            extension,
        } => execute_mint_reward(deps, env, info, user_addr, token_id, token_uri, extension),
        ExecuteMsg::UpdateUserInfo {
            address,
            level,
            total_xp,
        } => execute_update_user_info(deps, env, info, address, level, total_xp),
    }
}

fn execute_mint_reward(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_addr: String,
    token_id: String,
    token_uri: Option<String>,
    extension: Option<Metadata>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    // Call mint msg on reward contract
    let reward_contract = REWARD_CONTRACT.load(deps.storage)?;
    let mint_msg = WasmMsg::Execute {
        contract_addr: reward_contract.to_string(),
        msg: to_json_binary(&cw2981_royalties::ExecuteMsg::Mint {
            token_id: token_id.clone(),
            owner: user_addr.clone(),
            token_uri: token_uri,
            extension: extension,
        })?,
        funds: vec![],
    };
    Ok(Response::new()
        .add_message(mint_msg)
        .add_attribute("action", "mint_reward")
        .add_attribute("user_addr", user_addr.to_string())
        .add_attribute("token_id", token_id.to_string()))
}

fn execute_update_user_info(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: String,
    level: u64,
    total_xp: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }
    let new_user_info = UserInfo {
        address: deps.api.addr_validate(&address)?,
        level,
        total_xp,
    };
    USER_INFOS.save(deps.storage, &address, &new_user_info)?;

    Ok(Response::new()
        .add_attribute("action", "update_user_info")
        .add_attribute("user_addr", address.to_string())
        .add_attribute("level", level.to_string())
        .add_attribute("total_xp", total_xp.to_string()))
}

fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    admin: String,
) -> Result<Response, ContractError> {
    // only contract admin can update config
    let config = CONFIG.load(deps.storage)?;
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // update config
    let new_config = Config {
        admin: deps.api.addr_validate(&admin)?,
    };
    CONFIG.save(deps.storage, &new_config)?;
    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("admin", admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        REPLY_ID => {
            let reply = parse_reply_instantiate_data(msg).unwrap();

            let reward_contract = deps.api.addr_validate(&reply.contract_address)?;
            REWARD_CONTRACT.save(deps.storage, &reward_contract)?;

            Ok(Response::new().add_attribute("reward_contract", reward_contract))
        }
        _ => Err(StdError::NotFound {
            kind: format!("not found reply id {:?}", msg.id),
        }),
    }
}

#[cfg(test)]
mod tests {}
