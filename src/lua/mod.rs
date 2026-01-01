pub mod var;
pub mod func;

use std::{collections::HashMap, sync::{Mutex, OnceLock}};
use mlua::prelude::*;

use crate::shared::func::Func;

/// This is the Lua state. Each language gets it's own private state
struct State {
    /// The lua engine.
    engine: Lua,
    /// The function hash for lua callbacks.
    function_hash: HashMap<u32, Func>
}

/// The State static variable for Lua.
static STATE: OnceLock<State> = OnceLock::new();

/// Get the state of LUA.
fn get_state() -> &mut State {
    let mutex = STATE.get_mut().unwrap();
    mutex
}

/// Execute some orbituary lua code.
/// Returns a String. Empty means no error happened and was successful!
pub fn execute(code: &str, file_name: &str) -> String {
    let state = get_state();
    let res = state.engine.load(code).exec();
    if res.is_err() {
        return res.unwrap_err().to_string();
    }

    String::from("")
}