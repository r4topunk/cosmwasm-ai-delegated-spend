pub mod contract;
pub mod msg {
    pub mod init;
    pub mod exec;
    pub mod query;
}
pub mod state;
pub mod error;

pub use contract::{instantiate, execute, query};
pub use msg::init::InstantiateMsg;
pub use msg::exec::ExecuteMsg;
pub use msg::query::QueryMsg;
