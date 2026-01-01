use super::var::Var;
use std::ffi::c_void;

/// Function reference used in C.
/// 
/// argc: i32, The number of args.
/// argc: *const Var, a C Array of args.
/// opaque: *mut c_void, opaque user data. 
pub type Func = unsafe extern "C" fn(argc: i32, argv: *const Var, opaque: *mut c_void) -> *mut Var;

/// Rust Function reference which wraps the C Func.
pub struct Function {
    /// The C Function wrapper.
    func: Func,
    /// The idx, gotta be greater than -1, hence u32
    idx: u32
}