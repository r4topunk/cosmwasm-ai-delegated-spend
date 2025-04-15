/// Query logic for the Credits Delegation contract
///
/// This module handles all read-only operations for the contract,
/// allowing clients to retrieve information about balances and authorizations
/// without modifying contract state.
use cosmwasm_std::{Deps, Env, StdResult, Binary, to_json_binary};
use crate::msg::query::QueryMsg;
use crate::state::{BALANCES, AUTHORIZED_SPENDERS};

/// Main entry point for all query messages
///
/// Routes incoming query messages to the appropriate handler function
/// and serializes the response into JSON binary format.
///
/// # Arguments
/// * `deps` - Dependencies for storage, API, and querier access
/// * `_env` - Environment information (block height/time, contract address)
/// * `msg` - The query message specifying what information to retrieve
///
/// # Returns
/// * `StdResult<Binary>` - JSON-serialized query result or error
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { owner } => query_balance(deps, owner),
        QueryMsg::IsAuthorized { owner, spender } => query_is_authorized(deps, owner, spender),
    }
}

/// Queries the balance of a specific address
///
/// Returns the current token balance for the specified owner address.
/// If the address has no balance record, returns 0.
///
/// # Arguments
/// * `deps` - Dependencies for storage access and address validation
/// * `owner` - Address string of the account to check balance for
///
/// # Returns
/// * `StdResult<Binary>` - JSON-serialized balance as u128
fn query_balance(deps: Deps, owner: String) -> StdResult<Binary> {
    // Validate the owner address
    let owner_addr = deps.api.addr_validate(&owner)?;
    
    // Look up balance in state, defaulting to 0 if not found
    let balance = BALANCES.may_load(deps.storage, &owner_addr)?.unwrap_or(0);
    
    // Return the serialized balance
    to_json_binary(&balance)
}

/// Checks if a spender is authorized by an owner
///
/// Verifies whether the spender address has been granted spending permission
/// by the owner address via the AuthorizeSpender message.
///
/// # Arguments
/// * `deps` - Dependencies for storage access and address validation
/// * `owner` - Address string of the token owner
/// * `spender` - Address string of the potential spender
///
/// # Returns
/// * `StdResult<Binary>` - JSON-serialized boolean (true if authorized)
fn query_is_authorized(deps: Deps, owner: String, spender: String) -> StdResult<Binary> {
    // Validate both addresses
    let owner_addr = deps.api.addr_validate(&owner)?;
    let spender_addr = deps.api.addr_validate(&spender)?;
    
    // Check authorization status in state, defaulting to false if not found
    let authorized = AUTHORIZED_SPENDERS
        .may_load(deps.storage, (&owner_addr, &spender_addr))?
        .unwrap_or(false);
    
    // Return the serialized authorization status
    to_json_binary(&authorized)
}
