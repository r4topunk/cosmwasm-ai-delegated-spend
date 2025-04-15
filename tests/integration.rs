use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::coins;
use credits_delegation::{instantiate, execute, query};
use credits_delegation::msg::init::InstantiateMsg;
use credits_delegation::msg::exec::ExecuteMsg;
use credits_delegation::msg::query::QueryMsg;

#[test]
fn test_deposit_and_spend_flow() {
    let mut deps = mock_dependencies();
    let admin = "admin";
    let denom = "ucosm";
    let instantiate_msg = InstantiateMsg { admin: admin.to_string(), denom: denom.to_string() };
    let info = mock_info(admin, &[]);
    instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

    // User deposits
    let user = "user1";
    let deposit_info = mock_info(user, &coins(1000, denom));
    execute(deps.as_mut(), mock_env(), deposit_info, ExecuteMsg::Deposit {}).unwrap();

    // Authorize spender
    let spender = "spender1";
    let auth_info = mock_info(user, &[]);
    execute(deps.as_mut(), mock_env(), auth_info, ExecuteMsg::AuthorizeSpender { spender: spender.to_string() }).unwrap();

    // Spender spends from user
    let spend_info = mock_info(spender, &[]);
    execute(deps.as_mut(), mock_env(), spend_info, ExecuteMsg::SpendFrom { owner: user.to_string(), amount: 500 }).unwrap();

    // Query balances
    let balance: u128 = cosmwasm_std::from_json(&query(deps.as_ref(), mock_env(), QueryMsg::Balance { owner: user.to_string() }).unwrap()).unwrap();
    assert_eq!(balance, 500);
    let spender_balance: u128 = cosmwasm_std::from_json(&query(deps.as_ref(), mock_env(), QueryMsg::Balance { owner: spender.to_string() }).unwrap()).unwrap();
    assert_eq!(spender_balance, 500);
    // Query authorization
    let is_auth: bool = cosmwasm_std::from_json(&query(deps.as_ref(), mock_env(), QueryMsg::IsAuthorized { owner: user.to_string(), spender: spender.to_string() }).unwrap()).unwrap();
    assert!(is_auth);
}
