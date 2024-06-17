#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cw2::set_contract_version ;
use crate::state::{Config, CONFIG, AGREEMENT, Agreement};

const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version( deps.storage, CONTRACT_NAME, CONTRACT_VERSION)? ;
    let validated_admin_address = deps.api.addr_validate(&msg.admin_address)?;

    let config = Config {
        admin_address: validated_admin_address,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg :: CreateAgreement { duration, amount } => execute_create_agreement(deps, env, info, duration, amount),
    }
}

fn execute_create_agreement(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    duration: u64,
    amount: i128,
) -> Result<Response, ContractError> {
    let agreement = Agreement {
        id: 0,
        owner: info.sender,
        price: 0,
        initial_amount: amount,
        final_amount: 0,
        start_time: env.block.time.seconds(),
        end_time: env.block.time.seconds() + duration,
        withdrawn: false,
        on_market: false,
        timeleft: duration,
    };

    AGREEMENT.save(deps.storage, duration, &agreement)?;

    Ok(Response::new().add_attribute("action", "create_agreement"))
}

fn execute_withdraw_agreement(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _duration: Timestamp,
    _amount: i128,
) -> Result<Response, ContractError> {
    unimplemented!()

}

fn execute_buy_agreement(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _duration: Timestamp,
    _amount: i128,
) -> Result<Response, ContractError> {
    unimplemented!()
}

fn execute_sell_agreement(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _duration: Timestamp,
    _amount: i128,
) -> Result<Response, ContractError> {
    unimplemented!()
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

fn query_view_all_agreements(
    _deps: Deps,
    _env: Env,
    _msg: QueryMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

fn query_view_active_agreement(
    _deps: Deps,
    _env: Env,
    _msg: QueryMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use crate::msg::InstantiateMsg;

    use super::instantiate;

    #[test]
    fn test_instantiate() {
        let mut deps  = mock_dependencies();
        let env = mock_env();   
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(),
        };

        let resp = instantiate(deps.as_mut(), env, info, msg).unwrap() ;
        assert_eq!(resp.attributes, vec![("action", "instantiate")])
        }
    }
