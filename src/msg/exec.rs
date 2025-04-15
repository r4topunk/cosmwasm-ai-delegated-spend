use cosmwasm_schema::cw_serde;

/// Execute messages for the Credits Delegation contract
///
/// These messages represent all actions that can modify the contract's state.
/// Each variant handles a different operation in the delegation system.
#[cw_serde]
pub enum ExecuteMsg {
    /// Deposits native tokens into the sender's balance
    /// 
    /// The deposited amount is determined by the funds sent with the transaction.
    /// Must include exactly one native token matching the contract's configured denom.
    Deposit {},
    
    /// Authorizes an address to spend tokens on behalf of the sender
    /// 
    /// After authorization, the spender can use SpendFrom to use tokens from the owner's balance.
    /// Only the token owner can authorize spenders for their account.
    AuthorizeSpender { spender: String },
    
    /// Removes spending authorization from a previously authorized address
    /// 
    /// After revocation, the spender can no longer spend tokens from the owner's balance.
    /// Only the token owner can revoke authorizations for their account.
    RevokeSpender { spender: String },
    
    /// Spends tokens from an owner's account to the sender's account
    /// 
    /// Can only be executed by either:
    /// 1. The owner themselves (self-spending)
    /// 2. An address previously authorized by the owner via AuthorizeSpender
    /// Fails if the owner has insufficient balance or if sender is unauthorized.
    SpendFrom { owner: String, amount: u128 },
}
