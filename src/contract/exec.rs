// Execution logic for contract
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::msg::exec::ExecuteMsg;
use crate::error::ContractError;
use crate::state::{BALANCES, AUTHORIZED_SPENDERS};

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => execute_deposit(deps, info),
        ExecuteMsg::AuthorizeSpender { spender } => execute_authorize_spender(deps, info, spender),
        ExecuteMsg::RevokeSpender { spender } => execute_revoke_spender(deps, info, spender),
        ExecuteMsg::SpendFrom { owner, amount } => execute_spend_from(deps, info, owner, amount),
    }
}

fn execute_deposit(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let amount = one_native_token(&info)?;
    let sender = info.sender;
    let prev = BALANCES.may_load(deps.storage, &sender)?.unwrap_or(0);
    BALANCES.save(deps.storage, &sender, &(prev + amount))?;
    Ok(Response::new().add_attribute("action", "deposit").add_attribute("from", sender))
}

fn execute_authorize_spender(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
) -> Result<Response, ContractError> {
    let owner = info.sender;
    let spender_addr = deps.api.addr_validate(&spender)?;
    AUTHORIZED_SPENDERS.save(deps.storage, (&owner, &spender_addr), &true)?;
    Ok(Response::new().add_attribute("action", "authorize_spender").add_attribute("owner", owner).add_attribute("spender", spender_addr))
}

fn execute_revoke_spender(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
) -> Result<Response, ContractError> {
    let owner = info.sender;
    let spender_addr = deps.api.addr_validate(&spender)?;
    AUTHORIZED_SPENDERS.remove(deps.storage, (&owner, &spender_addr));
    Ok(Response::new().add_attribute("action", "revoke_spender").add_attribute("owner", owner).add_attribute("spender", spender_addr))
}

fn execute_spend_from(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
    amount: u128,
) -> Result<Response, ContractError> {
    let spender = info.sender;
    let owner_addr = deps.api.addr_validate(&owner)?;
    // Only authorized spender or owner can spend
    let is_owner = spender == owner_addr;
    let is_authorized = AUTHORIZED_SPENDERS.may_load(deps.storage, (&owner_addr, &spender))?.unwrap_or(false);
    if !is_owner && !is_authorized {
        return Err(ContractError::Unauthorized {});
    }
    let mut balance = BALANCES.may_load(deps.storage, &owner_addr)?.unwrap_or(0);
    if balance < amount {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Insufficient balance")));
    }
    balance -= amount;
    BALANCES.save(deps.storage, &owner_addr, &balance)?;
    // Send tokens to spender
    // (In a real contract, you would use BankMsg to send tokens. Here, we just update balances.)
    let prev = BALANCES.may_load(deps.storage, &spender)?.unwrap_or(0);
    BALANCES.save(deps.storage, &spender, &(prev + amount))?;
    Ok(Response::new().add_attribute("action", "spend_from").add_attribute("owner", owner_addr).add_attribute("spender", spender).add_attribute("amount", amount.to_string()))
}

fn one_native_token(info: &MessageInfo) -> Result<u128, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Must send exactly one native token")));
    }
    Ok(info.funds[0].amount.u128())
}
