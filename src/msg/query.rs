use cosmwasm_schema::cw_serde;

/// Query messages for the Credits Delegation contract
///
/// These messages are used to retrieve information from the contract
/// without modifying its state. They provide read-only access to current
/// balances and authorization statuses.
#[cw_serde]
pub enum QueryMsg {
    /// Retrieves the token balance for a given address
    /// 
    /// Returns a u128 value representing the current balance.
    /// If the address has no recorded balance, returns 0.
    Balance { owner: String },
    
    /// Checks if a spender is authorized to spend on behalf of an owner
    /// 
    /// Returns a boolean value:
    /// - true if the spender is authorized by the owner
    /// - false if no authorization exists
    IsAuthorized { owner: String, spender: String },
}
