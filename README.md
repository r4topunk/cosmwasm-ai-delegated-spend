# Credits Delegation CosmWasm Smart Contract

## Overview
This contract implements a secure, modular delegated spending system for Cosmos blockchains using CosmWasm. It allows users to deposit credits, authorize spenders, and delegate spending rights, following best practices for security, modularity, and extensibility.

## Features
- **Secure Deposit System**: Users can deposit native tokens to their account balance
- **Delegation Control**: Token owners can authorize and revoke spending rights for other addresses
- **Spend On Behalf**: Authorized spenders can spend from owner accounts
- **Balance Queries**: Check user's current credit balance
- **Authorization Verification**: Verify if a spender is authorized by an owner

## Contract Architecture

### Core Components
- **State Management**: Tracks admin, token denomination, user balances, and authorized spenders
- **Message Handling**: Processes instantiation, execution, and query requests
- **Access Control**: Implements permission validation for spending operations

### Entry Points
- `instantiate`: Initializes the contract with an admin address and token denomination
- `execute`: Processes deposit, authorization, revocation, and spending operations
- `query`: Provides information about balances and authorization status

## Message Types

### Instantiation
- `InstantiateMsg`: Sets contract admin and the native token denomination used

### Execution
- `Deposit`: Adds deposited funds to user's balance
- `AuthorizeSpender`: Grants spending permission to another address
- `RevokeSpender`: Removes spending permission from an address
- `SpendFrom`: Allows spending tokens from an owner's account (if authorized)

### Queries
- `Balance`: Returns the token balance of a specified address
- `IsAuthorized`: Checks if a spender is authorized by an owner

## Usage Examples

### Instantiating the Contract
```rust
// Create a new contract instance with an admin and the accepted token denomination
let instantiate_msg = InstantiateMsg {
    admin: "cosmos1...".to_string(),
    denom: "uatom".to_string(),
};
```

### Depositing Tokens
```rust
// User deposits tokens by sending them with the Deposit message
let execute_msg = ExecuteMsg::Deposit {};
```

### Authorizing a Spender
```rust
// Owner authorizes another address to spend on their behalf
let execute_msg = ExecuteMsg::AuthorizeSpender {
    spender: "cosmos2...".to_string(),
};
```

### Spending From an Account
```rust
// Authorized spender spends tokens from owner's account
let execute_msg = ExecuteMsg::SpendFrom {
    owner: "cosmos1...".to_string(),
    amount: 100u128,
};
```

## Security Considerations
- All user inputs are validated using Cosmos SDK's address validation
- Authorization checks prevent unauthorized spending
- Balance validation ensures sufficient funds before spending

## Development

### Prerequisites
- Rust 1.65.0+
- [wasmd](https://github.com/CosmWasm/wasmd) for local testing
- [cargo-make](https://github.com/sagiegurari/cargo-make) for build scripts

### Building
```bash
cargo wasm
```

### Testing
```bash
cargo test
```

### Optimizing for Production
```bash
cargo wasm-opt
```
- **Deposit**: Users can deposit native tokens to their balance.
- **Authorize/Revoke Spender**: Users can grant or revoke permission for other accounts to spend on their behalf.
- **Delegated Spend**: Authorized spenders can transfer credits from the owner's balance.
- **Balance & Authorization Queries**: Query balances and authorization status.

## Project Structure
```
contract/
  exec.rs         # Execution logic (deposit, authorize, revoke, spend)
  init.rs         # Initialization logic
  mod.rs          # Public contract interface
  query.rs        # Query logic
error.rs          # Custom error type
lib.rs            # Crate entrypoint
msg/
  exec.rs         # ExecuteMsg definitions
  init.rs         # InstantiateMsg definition
  query.rs        # QueryMsg definitions
schema.rs         # JSON schema generator
state.rs          # State storage (admin, denom, balances, authorizations)
tests_integration.rs # Integration tests (cw-multi-test)
Cargo.toml        # Dependencies
```

## Security & Best Practices
- Strict access control and input validation
- No reentrancy or unchecked math
- Uses only audited dependencies
- Modular, maintainable code

## Usage
### Build & Test
```fish
cargo test
```

### Generate Optimized Wasm
```fish
cargo wasm && wasm-opt -Oz -o contract.wasm target/wasm32-unknown-unknown/release/credits_delegation.wasm
```

### Generate JSON Schema
```fish
cargo run --bin schema
```

## Example Integration Test
See `tests_integration.rs` for a full deposit, authorization, and spend flow.

## License
MIT
