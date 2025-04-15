/// Main contract entry points and implementation
/// 
/// This module organizes the contract's core logic into separate files by functionality:
/// - `init.rs`: Contract instantiation logic
/// - `exec.rs`: Execution message handling
/// - `query.rs`: Query message handling
///
/// The separation allows for better code organization while maintaining a clean public API
/// through re-exports of the main entry point functions.

pub mod init;
pub mod exec;
pub mod query;

// Re-export public interfaces for easier imports by consuming code
pub use init::*;
pub use exec::*;
pub use query::*;
