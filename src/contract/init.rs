/// Initialization logic for the Credits Delegation contract
///
/// This module handles the instantiation of the contract, validating and storing
/// the initial configuration parameters.
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use crate::msg::init::InstantiateMsg;
use crate::state::{ADMIN, DENOM};

/// Instantiates a new Credits Delegation contract
///
/// This function is called exactly once when the contract is first deployed.
/// It sets up the initial contract state by:
/// 1. Validating the admin address
/// 2. Saving the admin address to state
/// 3. Saving the accepted token denomination to state
///
/// # Arguments
/// * `deps` - Mutable dependencies for storage, API, and querier access
/// * `_env` - Environment information (block height/time, contract address)
/// * `_info` - Transaction metadata (sender, sent funds)
/// * `msg` - Instantiation parameters (admin address, token denom)
///
/// # Returns
/// * `StdResult<Response>` - Success response or error
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Validate that the admin address is a proper bech32 address
    let admin = deps.api.addr_validate(msg.admin.as_str())?;
    
    // Save admin address to contract state
    ADMIN.save(deps.storage, &admin)?;
    
    // Save token denomination to contract state
    DENOM.save(deps.storage, &msg.denom)?;
    
    // Return success response with method attribute
    Ok(Response::new().add_attribute("method", "instantiate"))
}
