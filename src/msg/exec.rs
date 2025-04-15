use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {},
    AuthorizeSpender { spender: String },
    RevokeSpender { spender: String },
    SpendFrom { owner: String, amount: u128 },
}
