#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/stm32_hal_bindings.rs"));
// TODO separate HAL from statics
include!(concat!(env!("OUT_DIR"), "/stm32_hal_statics.rs"));
