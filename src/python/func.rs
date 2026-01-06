use rustpython::vm::{Context, PyRef, VirtualMachine, function::PyNativeFn};

// Attach a function to a context.
pub(super) fn attach_function(vm: VirtualMachine, ctx: Context, fn_name: &str, fn_idx: i32, obj: Option<i32>) {
    
}
// pub(super) fn internal_add_callback(lua: &Lua, fn_idx: i32, obj: Option<i32>) -> LuaFunction {
