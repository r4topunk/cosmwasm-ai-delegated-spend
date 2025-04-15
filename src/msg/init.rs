use cosmwasm_schema::cw_serde;

/// Message for instantiating the Credits Delegation contract
///
/// This message is sent once when the contract is first deployed to initialize
/// its state. It configures who the admin is and what token denomination is accepted.
#[cw_serde]
pub struct InstantiateMsg {
    /// Address that will be set as the contract administrator
    /// Must be a valid bech32 address string that will be validated during instantiation
    pub admin: String,
    
    /// Native token denomination that the contract will accept for deposits
    /// Example: "uatom" for Cosmos Hub atoms
    pub denom: String,
}
