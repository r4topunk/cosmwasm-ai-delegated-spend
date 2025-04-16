use cosmwasm_std::{testing::{mock_dependencies, mock_env, mock_info}, coins};
use credits_delegation::{instantiate, execute, query};
use credits_delegation::msg::init::InstantiateMsg;
use credits_delegation::msg::exec::ExecuteMsg;
use credits_delegation::msg::query::QueryMsg;

/// # Credits Delegation Contract Testing Guide
///
/// This file contains comprehensive integration tests for the Credits Delegation contract.
/// Each test simulates a different scenario or edge case to verify the contract's behavior.
///
/// ## Testing Approach
/// The tests use CosmWasm's mock environment, which simulates blockchain behavior without
/// requiring an actual blockchain. This allows for rapid testing of contract logic.
///
/// ## Key Testing Concepts
/// - `mock_dependencies()`: Creates a simulated blockchain backend
/// - `mock_env()`: Sets up a simulated blockchain environment
/// - `mock_info()`: Creates transaction context (sender, funds)
///
/// ## Learning Points
/// These tests demonstrate:
/// 1. Contract instantiation and configuration
/// 2. Token deposits and balance tracking
/// 3. Access control mechanisms
/// 4. Delegated spending authorization
/// 5. Error handling and validation

/// ## Happy Path Test
/// Tests the complete flow of deposit, authorization, and spending
/// to verify that the core functionality works correctly.
#[test]
fn test_deposit_and_spend_flow() {
    // Set up test environment with mock blockchain dependencies
    let mut deps = mock_dependencies();
    
    // Define contract configuration
    let admin = "admin";
    let denom = "ucosm";
    
    // Instantiate contract with admin and token denomination
    let instantiate_msg = InstantiateMsg { admin: admin.to_string(), denom: denom.to_string() };
    let info = mock_info(admin, &[]);  // No funds sent with instantiation
    instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

    // User deposits 1000 tokens
    let user = "user1";
    let deposit_info = mock_info(user, &coins(1000, denom));  // Include tokens with message
    execute(deps.as_mut(), mock_env(), deposit_info, ExecuteMsg::Deposit {}).unwrap();

    // User authorizes a spender to access their funds
    let spender = "spender1";
    let auth_info = mock_info(user, &[]);  // No funds needed for authorization
    execute(deps.as_mut(), mock_env(), auth_info, ExecuteMsg::AuthorizeSpender { spender: spender.to_string() }).unwrap();

    // Spender transfers 500 tokens from user's balance to their own
    let spend_info = mock_info(spender, &[]);
    execute(deps.as_mut(), mock_env(), spend_info, ExecuteMsg::SpendFrom { owner: user.to_string(), amount: 500 }).unwrap();

    // Verify user's remaining balance (1000 - 500 = 500)
    let balance: u128 = cosmwasm_std::from_json(&query(deps.as_ref(), mock_env(), QueryMsg::Balance { owner: user.to_string() }).unwrap()).unwrap();
    assert_eq!(balance, 500);
    
    // Verify spender received the tokens (0 + 500 = 500)
    let spender_balance: u128 = cosmwasm_std::from_json(&query(deps.as_ref(), mock_env(), QueryMsg::Balance { owner: spender.to_string() }).unwrap()).unwrap();
    assert_eq!(spender_balance, 500);
    
    // Verify that the authorization is still valid after the spend
    let is_auth: bool = cosmwasm_std::from_json(&query(deps.as_ref(), mock_env(), QueryMsg::IsAuthorized { owner: user.to_string(), spender: spender.to_string() }).unwrap()).unwrap();
    assert!(is_auth);
}

/// ## Deposit Validation Test
/// Tests that only the correct token denomination is accepted
/// and proper error handling for invalid deposits.
#[test]
fn test_deposit_validation() {
    let mut deps = mock_dependencies();
    
    // Initialize contract to accept only "uatom" tokens
    let admin = "admin";
    let denom = "uatom"; 
    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(admin, &[]),
        InstantiateMsg { admin: admin.to_string(), denom: denom.to_string() }
    ).unwrap();

    // Case 1: Deposit with correct denomination
    let user = "user1";
    let deposit_result = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user, &coins(500, denom)),
        ExecuteMsg::Deposit {}
    );
    assert!(deposit_result.is_ok());
    
    // Verify balance was recorded
    let balance: u128 = cosmwasm_std::from_json(&query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Balance { owner: user.to_string() }
    ).unwrap()).unwrap();
    assert_eq!(balance, 500);

    // Case 2: Deposit with wrong denomination
    let wrong_denom_result = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user, &coins(300, "usdt")),  // Wrong token denom
        ExecuteMsg::Deposit {}
    );
    // This should error as wrong token denom provided
    assert!(wrong_denom_result.is_err());
    
    // Case 3: Deposit with multiple coin types
    let multiple_coins_result = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user, &[coins(300, denom).first().unwrap().clone(), coins(100, "usdt").first().unwrap().clone()]),
        ExecuteMsg::Deposit {}
    );
    // This should error as multiple token denoms provided
    assert!(multiple_coins_result.is_err());
    
    // Verify balance hasn't changed after failed attempts
    let balance_after: u128 = cosmwasm_std::from_json(&query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Balance { owner: user.to_string() }
    ).unwrap()).unwrap();
    assert_eq!(balance_after, 500);
}

/// ## Authorization Test
/// Tests the authorization and revocation mechanisms
/// for delegated spending permissions.
#[test]
fn test_authorization_and_revocation() {
    let mut deps = mock_dependencies();
    
    // Setup contract
    let admin = "admin";
    let denom = "uluna";
    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(admin, &[]),
        InstantiateMsg { admin: admin.to_string(), denom: denom.to_string() }
    ).unwrap();
    
    // Setup user with balance
    let user = "user1";
    let spender = "spender1";
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user, &coins(1000, denom)),
        ExecuteMsg::Deposit {}
    ).unwrap();
    
    // Test 1: Initial state - spender should NOT be authorized
    let initial_auth: bool = cosmwasm_std::from_json(&query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::IsAuthorized { owner: user.to_string(), spender: spender.to_string() }
    ).unwrap()).unwrap();
    assert_eq!(initial_auth, false);
    
    // Test 2: Authorize spender
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user, &[]),
        ExecuteMsg::AuthorizeSpender { spender: spender.to_string() }
    ).unwrap();
    
    // Verify authorization was granted
    let auth_granted: bool = cosmwasm_std::from_json(&query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::IsAuthorized { owner: user.to_string(), spender: spender.to_string() }
    ).unwrap()).unwrap();
    assert_eq!(auth_granted, true);
    
    // Test 3: Revoke authorization
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user, &[]),
        ExecuteMsg::RevokeSpender { spender: spender.to_string() }
    ).unwrap();
    
    // Verify authorization was revoked
    let auth_revoked: bool = cosmwasm_std::from_json(&query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::IsAuthorized { owner: user.to_string(), spender: spender.to_string() }
    ).unwrap()).unwrap();
    assert_eq!(auth_revoked, false);
    
    // Test 4: Only owner can authorize/revoke
    let other_user = "other_user";
    let unauthorized_auth = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(other_user, &[]),
        ExecuteMsg::AuthorizeSpender { spender: spender.to_string() }
    );
    
    // This should return an error since other_user is trying to authorize on behalf of user
    assert!(unauthorized_auth.is_err());
}

/// ## Spending Authorization Test
/// Tests that only authorized spenders can spend from an owner's balance
/// and validates proper error handling for unauthorized access.
#[test]
fn test_spending_authorization() {
    let mut deps = mock_dependencies();
    
    // Setup contract
    let admin = "admin";
    let denom = "ujuno";
    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(admin, &[]),
        InstantiateMsg { admin: admin.to_string(), denom: denom.to_string() }
    ).unwrap();
    
    // Setup users with balances
    let owner = "owner";
    let authorized_spender = "auth_spender";
    let unauthorized_spender = "unauth_spender";
    
    // Owner deposits funds
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(owner, &coins(1000, denom)),
        ExecuteMsg::Deposit {}
    ).unwrap();
    
    // Owner authorizes one spender but not the other
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(owner, &[]),
        ExecuteMsg::AuthorizeSpender { spender: authorized_spender.to_string() }
    ).unwrap();
    // Test 1: Authorized spender can spend
    let auth_spend_result = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(authorized_spender, &[]),
        ExecuteMsg::SpendFrom { owner: owner.to_string(), amount: 300 }
    );
    assert!(auth_spend_result.is_ok());
    
    // Verify balances after authorized spend
    let owner_balance: u128 = cosmwasm_std::from_json(&query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Balance { owner: owner.to_string() }
    ).unwrap()).unwrap();
    assert_eq!(owner_balance, 700); // 1000 - 300
    
    let auth_spender_balance: u128 = cosmwasm_std::from_json(&query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Balance { owner: authorized_spender.to_string() }
    ).unwrap()).unwrap();
    assert_eq!(auth_spender_balance, 300);
    
    // Test 2: Unauthorized spender cannot spend
    let unauth_spend_result = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(unauthorized_spender, &[]),
        ExecuteMsg::SpendFrom { owner: owner.to_string(), amount: 100 }
    );
    assert!(unauth_spend_result.is_err());
    
    // Verify balances remain unchanged after unauthorized attempt
    let owner_balance_after: u128 = cosmwasm_std::from_json(&query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Balance { owner: owner.to_string() }
    ).unwrap()).unwrap();
    assert_eq!(owner_balance_after, 700); // Still 700
}

/// ## Overdraft Protection Test
/// Tests that the contract prevents spending more than available balance.
#[test]
fn test_overdraft_protection() {
    let mut deps = mock_dependencies();
    
    // Setup contract
    let admin = "admin";
    let denom = "uosmo";
    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(admin, &[]),
        InstantiateMsg { admin: admin.to_string(), denom: denom.to_string() }
    ).unwrap();
    
    // Setup user with limited balance
    let user = "user1";
    let spender = "spender1";
    
    // User deposits 500 tokens
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user, &coins(500, denom)),
        ExecuteMsg::Deposit {}
    ).unwrap();
    
    // Authorize spender
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user, &[]),
        ExecuteMsg::AuthorizeSpender { spender: spender.to_string() }
    ).unwrap();
    
    // Test 1: Spend exactly what's available
    let exact_spend = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(spender, &[]),
        ExecuteMsg::SpendFrom { owner: user.to_string(), amount: 500 }
    );
    assert!(exact_spend.is_ok());
    
    // Test 2: Attempt to spend more after account is empty
    let overdraft_result = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(spender, &[]),
        ExecuteMsg::SpendFrom { owner: user.to_string(), amount: 1 }
    );
    assert!(overdraft_result.is_err());
    
    // Test 3: User deposits more funds
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user, &coins(1000, denom)),
        ExecuteMsg::Deposit {}
    ).unwrap();
    
    // Test 4: Attempt to spend more than available
    let excessive_spend = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(spender, &[]),
        ExecuteMsg::SpendFrom { owner: user.to_string(), amount: 2000 }
    );
    assert!(excessive_spend.is_err());
    
    // Balance should still be intact
    let balance: u128 = cosmwasm_std::from_json(&query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Balance { owner: user.to_string() }
    ).unwrap()).unwrap();
    assert_eq!(balance, 1000);
}

/// ## Self-Spending Test
/// Tests that an owner can spend their own funds without explicit authorization.
#[test]
fn test_self_spending() {
    let mut deps = mock_dependencies();
    
    // Setup contract
    let admin = "admin";
    let denom = "ustars";
    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(admin, &[]),
        InstantiateMsg { admin: admin.to_string(), denom: denom.to_string() }
    ).unwrap();
    
    // Setup user with balance
    let user = "user1";
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user, &coins(1000, denom)),
        ExecuteMsg::Deposit {}
    ).unwrap();
    
    // User spends their own funds (should work without authorization)
    let self_spend = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user, &[]),
        ExecuteMsg::SpendFrom { owner: user.to_string(), amount: 300 }
    );
    
    assert!(self_spend.is_ok());
    
    // Check that balance was adjusted (spent tokens become a wash)
    let balance: u128 = cosmwasm_std::from_json(&query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::Balance { owner: user.to_string() }
    ).unwrap()).unwrap();
    assert_eq!(balance, 1000); // Balance remains at 1000 because self-spending is effectively a no-op
}
