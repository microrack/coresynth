use crate::hal::stm32_hal::bindings::GPIO_TypeDef;

include!(concat!(env!("OUT_DIR"), "/glue_bindings.rs"));

// bindgen can't handle defines with type casts
// See stm32f1xx_hal_tim.h

pub const TIM_CHANNEL_1: u32 = 0x0000;
pub const TIM_CHANNEL_2: u32 = 0x0004;
pub const TIM_CHANNEL_3: u32 = 0x0008;
pub const TIM_CHANNEL_4: u32 = 0x000C;
