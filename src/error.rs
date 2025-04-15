use thiserror::Error;
use cosmwasm_std::StdError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("Not implemented")]
    NotImplemented {},
    #[error(transparent)]
    Std(#[from] StdError),
}
