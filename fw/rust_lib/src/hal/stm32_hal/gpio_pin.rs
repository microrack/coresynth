use super::bindings::{
    HAL_GPIO_TogglePin,
    HAL_GPIO_WritePin,
    HAL_GPIO_ReadPin,
    GPIO_TypeDef,
    GPIO_PinState,
};

use crate::glue::gpio_init;

use crate::os::Mutex;
use crate::peripheral::Static;

use crate::hal::traits::{Pin, PinState, Error, Result, GpioMode};

pub struct GPIOPin {
    gpio_port: *mut GPIO_TypeDef,
    pin: u16,
}

impl Pin for GPIOPin {
    fn write(&mut self, state: PinState) {
        let stm_state = match state {
            PinState::Reset => GPIO_PinState::GPIO_PIN_RESET,
            PinState::Set => GPIO_PinState::GPIO_PIN_SET,
        };
        unsafe { HAL_GPIO_WritePin(self.gpio_port, self.pin, stm_state) };
    }

    fn toggle(&mut self) {
        unsafe { HAL_GPIO_TogglePin(self.gpio_port, self.pin) };
    }

    fn read(&self) -> PinState {
        let stm_state = unsafe {
            HAL_GPIO_ReadPin(self.gpio_port, self.pin)
        };
        match stm_state {
            GPIO_PinState::GPIO_PIN_RESET => PinState::Reset,
            GPIO_PinState::GPIO_PIN_SET => PinState::Set,
        }
    }

    fn mode(&self, mode: GpioMode) {
        unsafe { gpio_init(self.gpio_port, self.pin, mode as u32) };
    }
}


// pub static _PIN: Static<Mutex<GPIOPin>> = Static::new();

fn init_pin(
    statik: &Static<Mutex<GPIOPin>>,
    gpio_port: *mut GPIO_TypeDef,
    pin: u16,
    mode: GpioMode
) -> Result<()> {

    unsafe { gpio_init(gpio_port, pin, mode as u32) };

    let pin = GPIOPin {
        gpio_port,
        pin,
    };
    let mutex = Mutex::new(pin)
        .map_err(|_| Error {
            call: "GPIO init static Mutex::new",
        })?;
    statik.init(mutex);

    Ok(())
}

macro_rules! init_pin {
    ($id:ident) => {
        {
            use super::bindings::*;
            init_pin(&concat_idents!($id, _PIN), unsafe { concat_idents!(HAL_, $id, _GPIO_Port) }, concat_idents!(HAL_, $id, _Pin))?;
        }
    };
}

pub fn init_pins() -> Result<()> {
    // init_pin!(_PIN);

    Ok(())
}
