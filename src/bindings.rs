#![allow(safe_packed_borrows, non_camel_case_types, non_upper_case_globals, non_snake_case, improper_ctypes)]

use crate::c_types;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
