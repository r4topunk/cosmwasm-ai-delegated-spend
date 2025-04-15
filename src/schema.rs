use cosmwasm_schema::write_api;
use credits_delegation::msg::init::InstantiateMsg;
use credits_delegation::msg::exec::ExecuteMsg;
use credits_delegation::msg::query::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
