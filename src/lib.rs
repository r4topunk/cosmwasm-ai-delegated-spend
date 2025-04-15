/// Credits Delegation Contract - A CosmWasm smart contract for delegated token spending
///
/// This contract allows users to deposit native tokens, authorize other addresses to spend
/// on their behalf, and manage delegation permissions. It demonstrates secure authorization
/// patterns and state management in CosmWasm.
///
/// ## Module Structure
/// - `contract`: Contains the core contract logic divided into initialization,
///   execution, and query operations
/// - `msg`: Message type definitions for contract interaction
/// - `state`: State storage definitions
/// - `error`: Custom error handling

pub mod contract;
pub mod msg {
    pub mod init;
    pub mod exec;
    pub mod query;
}
pub mod state;
pub mod error;

// Re-export public interfaces for easier imports by consuming code
pub use contract::{instantiate, execute, query};
pub use msg::init::InstantiateMsg;
pub use msg::exec::ExecuteMsg;
pub use msg::query::QueryMsg;
