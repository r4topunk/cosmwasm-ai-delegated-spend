use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

/// Contract admin address with special privileges (if needed for future extensions)
pub const ADMIN: Item<Addr> = Item::new("admin");

/// Native token denomination that this contract accepts for deposits
pub const DENOM: Item<String> = Item::new("denom");

/// Maps user addresses to their token balances
/// Key: user address, Value: token balance as u128
pub const BALANCES: Map<&Addr, u128> = Map::new("balances");

/// Authorization mapping between owners and spenders
/// Key: (owner address, spender address), Value: authorization status (true/false)
/// Used to track which addresses are allowed to spend on behalf of owners
pub const AUTHORIZED_SPENDERS: Map<(&Addr, &Addr), bool> = Map::new("authorized_spenders");
