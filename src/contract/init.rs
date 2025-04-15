// Initialization logic for contract
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use crate::msg::init::InstantiateMsg;
use crate::state::{ADMIN, DENOM};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Validate admin address
    let admin = deps.api.addr_validate(msg.admin.as_str())?;
    ADMIN.save(deps.storage, &admin)?;
    DENOM.save(deps.storage, &msg.denom)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}
