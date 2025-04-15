use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const ADMIN: Item<Addr> = Item::new("admin");
pub const DENOM: Item<String> = Item::new("denom");
pub const BALANCES: Map<&Addr, u128> = Map::new("balances");
pub const AUTHORIZED_SPENDERS: Map<(&Addr, &Addr), bool> = Map::new("authorized_spenders");
