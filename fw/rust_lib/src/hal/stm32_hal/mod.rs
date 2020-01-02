pub mod gpio_pin;
mod release;
pub mod stm32_uart;

pub mod bindings;

use crate::hal::traits::Result;

pub fn init_statics() -> Result<()> {
    self::gpio_pin::init_pins()?;
    self::stm32_uart::debug_uart_init_static()?;
    Ok(())
}

pub use self::stm32_uart::{debug_uart_get};
