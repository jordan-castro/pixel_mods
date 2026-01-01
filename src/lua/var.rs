// Pure Rust goes here
use crate::{lua::get_state, shared::var::Var};

/// Add a variable by name to __main__ in lua.
pub fn add_variable(name: &str, variable: Var) {
    let state = get_state();

    match variable.tag {
        crate::shared::var::VarType::Int32 => {
            state.engine.globals().set(name, variable.get_i32()?);
        },
        crate::shared::var::VarType::Int64 => {
            state.engine.globals().set(name, variable.get_i64()?);
        },
        crate::shared::var::VarType::UInt32 => {
            state.engine.globals().set(name, variable.get_u32()?);
        },
        crate::shared::var::VarType::UInt64 => {
            state.engine.globals().set(name, variable.get_u64()?);
        },
        crate::shared::var::VarType::String => {
            state.engine.globals().set(name, variable.get_string()?);
        },
        crate::shared::var::VarType::Bool => {
            state.engine.globals().set(name, variable.get_bool()?);
        },
    }

    // Listo!
}