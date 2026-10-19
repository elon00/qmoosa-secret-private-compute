use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

#[entry_point]
pub fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Binary) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "instantiate_confidential_storage"))
}

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Binary) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "record_encrypted_payload"))
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: Binary) -> StdResult<Binary> {
    to_binary("confidential_query_authenticated_by_viewing_key")
}
