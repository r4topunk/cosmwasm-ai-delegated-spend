/// Execution logic for the Credits Delegation contract
///
/// This module handles all state-changing operations for the contract,
/// including deposits, authorization management, and token spending.
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::msg::exec::ExecuteMsg;
use crate::error::ContractError;
use crate::state::{BALANCES, AUTHORIZED_SPENDERS};

/// Main entry point for all execute messages
///
/// Routes incoming messages to the appropriate handler function based on the message type.
/// Each handler implements a specific piece of contract functionality.
///
/// # Arguments
/// * `deps` - Mutable dependencies for storage, API, and querier access
/// * `_env` - Environment information (block height/time, contract address)
/// * `info` - Transaction metadata (sender, sent funds)
/// * `msg` - The execute message with the operation to perform
///
/// # Returns
/// * `Result<Response, ContractError>` - Success response or error
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

/// Handles token deposits to the contract
///
/// Deposits sent tokens to the sender's balance in the contract.
/// This function validates that exactly one native token was sent,
/// then adds the amount to the sender's current balance.
///
/// # Arguments
/// * `deps` - Mutable dependencies for storage access
/// * `info` - Contains sender address and the funds sent with the transaction
///
/// # Returns
/// * `Result<Response, ContractError>` - Success response with event attributes or error
fn execute_deposit(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // Extract the amount from the sent funds
    let amount = one_native_token(&info)?;
    let sender = info.sender;
    
    // Update the sender's balance by adding the deposited amount
    let prev = BALANCES.may_load(deps.storage, &sender)?.unwrap_or(0);
    BALANCES.save(deps.storage, &sender, &(prev + amount))?;
    
    // Return success response with event attributes
    Ok(Response::new().add_attribute("action", "deposit").add_attribute("from", sender))
}

/// Authorizes a spender to spend on behalf of the message sender
///
/// Creates or updates an authorization record allowing the spender
/// to spend tokens from the sender's balance.
///
/// # Arguments
/// * `deps` - Mutable dependencies for storage access and address validation
/// * `info` - Contains the owner's address (message sender)
/// * `spender` - Address string of the account being authorized to spend
///
/// # Returns
/// * `Result<Response, ContractError>` - Success response with event attributes or error
fn execute_authorize_spender(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
) -> Result<Response, ContractError> {
    let owner = info.sender;
    // Validate that the spender address is a proper bech32 address
    let spender_addr = deps.api.addr_validate(&spender)?;
    
    // Save the authorization to state
    AUTHORIZED_SPENDERS.save(deps.storage, (&owner, &spender_addr), &true)?;
    
    // Return success response with event attributes
    Ok(Response::new().add_attribute("action", "authorize_spender").add_attribute("owner", owner).add_attribute("spender", spender_addr))
}

/// Revokes a spender's authorization to spend on behalf of the message sender
///
/// Removes an existing authorization record, preventing the spender from
/// spending tokens from the sender's balance.
///
/// # Arguments
/// * `deps` - Mutable dependencies for storage access and address validation
/// * `info` - Contains the owner's address (message sender)
/// * `spender` - Address string of the account having authorization revoked
///
/// # Returns
/// * `Result<Response, ContractError>` - Success response with event attributes or error
fn execute_revoke_spender(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
) -> Result<Response, ContractError> {
    let owner = info.sender;
    // Validate that the spender address is a proper bech32 address
    let spender_addr = deps.api.addr_validate(&spender)?;
    
    // Remove the authorization from state
    AUTHORIZED_SPENDERS.remove(deps.storage, (&owner, &spender_addr));
    
    // Return success response with event attributes
    Ok(Response::new().add_attribute("action", "revoke_spender").add_attribute("owner", owner).add_attribute("spender", spender_addr))
}

/// Spends tokens from an owner's account to the message sender's account
///
/// This function implements the core spending functionality, allowing either:
/// 1. An owner to spend from their own account
/// 2. An authorized spender to spend from the owner's account
///
/// The function verifies authorization, checks balance sufficiency,
/// updates the owner's balance, and credits the spender's account.
///
/// # Arguments
/// * `deps` - Mutable dependencies for storage access and address validation
/// * `info` - Contains the spender's address (message sender)
/// * `owner` - Address string of the account that owns the tokens
/// * `amount` - Number of tokens to spend
///
/// # Returns
/// * `Result<Response, ContractError>` - Success response with event attributes or error
fn execute_spend_from(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
    amount: u128,
) -> Result<Response, ContractError> {
    let spender = info.sender;
    let owner_addr = deps.api.addr_validate(&owner)?;
    
    // Verify spending authorization
    // Either the spender is the owner (self-spending) or has explicit authorization
    let is_owner = spender == owner_addr;
    let is_authorized = AUTHORIZED_SPENDERS.may_load(deps.storage, (&owner_addr, &spender))?.unwrap_or(false);
    if !is_owner && !is_authorized {
        return Err(ContractError::Unauthorized {});
    }
    
    // Check if owner has sufficient balance
    let mut balance = BALANCES.may_load(deps.storage, &owner_addr)?.unwrap_or(0);
    if balance < amount {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Insufficient balance")));
    }
    
    // Update owner's balance by subtracting the spent amount
    balance -= amount;
    BALANCES.save(deps.storage, &owner_addr, &balance)?;
    
    // Credit the tokens to the spender's account
    // Note: In a real contract with actual token transfers,
    // you might use BankMsg to send tokens instead
    let prev = BALANCES.may_load(deps.storage, &spender)?.unwrap_or(0);
    BALANCES.save(deps.storage, &spender, &(prev + amount))?;
    
    // Return success response with event attributes
    Ok(Response::new()
        .add_attribute("action", "spend_from")
        .add_attribute("owner", owner_addr)
        .add_attribute("spender", spender)
        .add_attribute("amount", amount.to_string()))
}

/// Utility function to extract and validate a single native token from the sent funds
///
/// This ensures that exactly one token type is sent with the transaction,
/// which simplifies accounting and prevents confusion about which tokens to use.
///
/// # Arguments
/// * `info` - Contains the funds sent with the transaction
///
/// # Returns
/// * `Result<u128, ContractError>` - The amount of tokens sent, or an error
fn one_native_token(info: &MessageInfo) -> Result<u128, ContractError> {
    // Validate that exactly one type of token was sent
    if info.funds.len() != 1 {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Must send exactly one native token")));
    }
    
    // Return the amount as a u128
    Ok(info.funds[0].amount.u128())
}
