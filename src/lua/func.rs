use std::{any::Any, ffi::c_void};

use mlua::{Lua, Variadic};

use crate::{lua::get_state, shared::{self, func::Func}};

// /// A Global LUA callback. This is what is used when registering callbacks.
// fn lua_callback(_:Lua, ) {

// }

/// Add a callback to lua __main__ context.
pub fn add_callback(name: &str, func: Func, opaque: *mut c_void) {
    let state = get_state();

    state.engine.create_function(|_, vars: Variadic<Any>| {
        // let res = func(1, )
        // println!("test!");

        Ok(())
    });
}