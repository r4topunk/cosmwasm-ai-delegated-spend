use thiserror::Error;
use cosmwasm_std::StdError;

/// Custom error types for the Credits Delegation contract
///
/// This enum defines all possible errors that can occur during contract execution.
/// Using a custom error type allows for more specific error handling and
/// better user feedback compared to generic errors.
#[derive(Error, Debug)]
pub enum ContractError {
    /// Returned when an unauthorized address attempts to perform a restricted operation
    /// Examples: non-authorized spender trying to spend tokens, non-admin trying admin operations
    #[error("Unauthorized")]
    Unauthorized {},
    
    /// Placeholder for features that are defined but not yet implemented
    #[error("Not implemented")]
    NotImplemented {},
    
    /// Wraps all standard CosmWasm errors for proper error propagation
    /// Examples: address validation errors, serialization errors, arithmetic errors
    #[error(transparent)]
    Std(#[from] StdError),
}
