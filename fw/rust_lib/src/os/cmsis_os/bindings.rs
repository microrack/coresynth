#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::ctypes::c_char;
use crate::ctypes::c_void;

include!(concat!(env!("OUT_DIR"), "/cmsis_os_bindings.rs"));
