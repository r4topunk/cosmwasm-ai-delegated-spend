// Query logic for contract
use cosmwasm_std::{Deps, Env, StdResult, Binary, to_json_binary};
use crate::msg::query::QueryMsg;
use crate::state::{BALANCES, AUTHORIZED_SPENDERS};

pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { owner } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            let balance = BALANCES.may_load(deps.storage, &owner_addr)?.unwrap_or(0);
            to_json_binary(&balance)
        }
        QueryMsg::IsAuthorized { owner, spender } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            let spender_addr = deps.api.addr_validate(&spender)?;
            let authorized = AUTHORIZED_SPENDERS.may_load(deps.storage, (&owner_addr, &spender_addr))?.unwrap_or(false);
            to_json_binary(&authorized)
        }
    }
}
