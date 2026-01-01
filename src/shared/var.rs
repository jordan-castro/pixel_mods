use std::ffi::{CStr, CString, c_char};

use anyhow::{Context, Error, anyhow};

/// Macro for writing out the Var:: get methods.
macro_rules! write_func {
    ($ (($func_name:ident, $field_name:ident, $ret_type:ty, $tag_variant:path) ),* $(,)?) => {
        $(
            #[doc = concat!("Returns the ", stringify!($ret_type), " value if the tag is ", stringify!($tag_variant), ".")]
            pub fn $func_name(&self) -> Result<$ret_type, Error> {
                if self.tag == $tag_variant {
                    unsafe {
                        Ok(self.value.$field_name)
                    }
                } else {
                    Err(anyhow!("Var is not the expected type of {}", $tag_variant))
                }
            }
        )*   
    };
}

// Macro for writing out the FromVars
macro_rules! implement_from_var {
    ($($t:ty, $func:ident);*) => {
        $(
            impl FromVar for $t {
                fn from_var(var:&Var) -> Result<Self, Error> {
                    var.$func()
                }
            }
        )*
    };
}

/// This represents the variable type that is being read or created.
#[repr(u32)]
#[derive(Debug)]
pub enum VarType {
    Int32,
    Int64,
    UInt32,
    UInt64,
    String,
    Bool,
    // Array,
    // // Python only
    // Dict,
    // // Lua only
    // Tree,
    // // JS/Python only
    // Object
}

/// The Variables actual value union.
#[repr(C)]
pub union VarValue {
    pub i32_val: i32,
    pub i64_val: i64,
    pub u32_val: u32,
    pub u64_val: u64,
    pub string_val: *mut c_char,
    pub bool_val: bool
}

/// The Variable struct that can be accessed directly from C.
#[repr(C)]
pub struct Var {
    pub tag: VarType,
    pub value: VarValue
}

// Rust specific functions
impl Var {
    pub fn get<T: FromVar>(&self) -> Result<T, Error> {
        T::from_var(self)
    }

    /// Get the Rust string from the Var.
    pub fn get_string(&self) -> Result<String, Error> {
        if self.tag == VarType::String {
            unsafe {
                if self.value.string_val.is_null() {
                    return Err(anyhow!("String pointer is null"));
                }

                let c_str = CStr::from_ptr(self.value.string_val);
                let res = c_str.to_str();
                if res.is_err() {
                    Err(anyhow!(res.err().unwrap()));
                }
                
                Ok(res.unwrap().to_string())
            }
        } else {
            Err(anyhow!("Var is not a string."))
        }
    }
    write_func!(
        (get_i32, i32_val, i32, VarType::Int32),
        (get_u32, u32_val, u32, VarType::UInt32),
        (get_i64, i64_val, i64, VarType::Int64),
        (get_u64, u64_val, u64, VarType::UInt64),
        (get_bool, bool_val, bool, VarType::Bool)
    );
}

/// Simple trait for Vars to get the type when writing code out.
pub trait FromVar: Sized {
    fn from_var(var: &Var) -> Result<Self, Error>;
}

implement_from_var! {
    i32, get_i32;
    u32, get_u32;
    String, get_string
}