# Credits Delegation CosmWasm Smart Contract

## Overview
This contract implements a secure, modular delegated spending system for Cosmos blockchains using CosmWasm. It allows users to deposit credits, authorize spenders, and delegate spending rights, following best practices for security, modularity, and extensibility.

## Features
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
