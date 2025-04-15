use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum QueryMsg {
    Balance { owner: String },
    IsAuthorized { owner: String, spender: String },
}
