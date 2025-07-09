use crate::msgs::{ExecuteMsg, InstantiateMsg};
use crate::state::Contract;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

impl<'a> Contract {
    pub fn instantiate(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> StdResult<Response> {
        Ok(Response::new())
    }

    pub fn execute(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: ExecuteMsg,
    ) -> StdResult<Response> {
        match msg {
            ExecuteMsg::Foo {} => Ok(Response::new()),
        }
    }
}
