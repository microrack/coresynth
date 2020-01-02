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

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum GPIO_PIN {
    GPIO_PIN_0  = 0x0001,
    GPIO_PIN_1  = 0x0002,
    GPIO_PIN_2  = 0x0004,
    GPIO_PIN_3  = 0x0008,
    GPIO_PIN_4  = 0x0010,
    GPIO_PIN_5  = 0x0020,
    GPIO_PIN_6  = 0x0040,
    GPIO_PIN_7  = 0x0080,
    GPIO_PIN_8  = 0x0100,
    GPIO_PIN_9  = 0x0200,
    GPIO_PIN_10 = 0x0400,
    GPIO_PIN_11 = 0x0800,
    GPIO_PIN_12 = 0x1000,
    GPIO_PIN_13 = 0x2000,
    GPIO_PIN_14 = 0x4000,
    GPIO_PIN_15 = 0x8000,
}

pub struct GPIOPin {
    gpio_port: *mut GPIO_TypeDef,
    pin: GPIO_PIN,
}

impl Pin for GPIOPin {
    fn write(&mut self, state: PinState) {
        let stm_state = match state {
            PinState::Reset => GPIO_PinState::GPIO_PIN_RESET,
            PinState::Set => GPIO_PinState::GPIO_PIN_SET,
        };
        unsafe { HAL_GPIO_WritePin(self.gpio_port, self.pin as u16, stm_state) };
    }

    fn toggle(&mut self) {
        unsafe { HAL_GPIO_TogglePin(self.gpio_port, self.pin as u16) };
    }

    fn read(&self) -> PinState {
        let stm_state = unsafe {
            HAL_GPIO_ReadPin(self.gpio_port, self.pin as u16)
        };
        match stm_state {
            GPIO_PinState::GPIO_PIN_RESET => PinState::Reset,
            GPIO_PinState::GPIO_PIN_SET => PinState::Set,
        }
    }

    fn mode(&self, mode: GpioMode) {
        unsafe { gpio_init(self.gpio_port, self.pin as u16, mode as u32) };
    }
}


pub static LED_0_PIN: Static<Mutex<GPIOPin>> = Static::new();
pub static LED_1_PIN: Static<Mutex<GPIOPin>> = Static::new();
pub static LED_2_PIN: Static<Mutex<GPIOPin>> = Static::new();
pub static LED_3_PIN: Static<Mutex<GPIOPin>> = Static::new();

fn init_pin(
    statik: &Static<Mutex<GPIOPin>>,
    gpio_port: *mut GPIO_TypeDef,
    pin: GPIO_PIN,
    mode: GpioMode
) -> Result<()> {

    unsafe { gpio_init(gpio_port, pin as u16, mode as u32) };

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


const PERIPH_BASE: u32 = 0x40000000;
const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x10000;

const GPIOA: *mut GPIO_TypeDef = (APB2PERIPH_BASE + 0x0800) as *mut GPIO_TypeDef;
const GPIOB: *mut GPIO_TypeDef = (APB2PERIPH_BASE + 0x0C00) as *mut GPIO_TypeDef;
const GPIOC: *mut GPIO_TypeDef = (APB2PERIPH_BASE + 0x1000) as *mut GPIO_TypeDef;

macro_rules! init_pin {
    ($id:ident) => {
        {
            use super::bindings::*;
            init_pin(&concat_idents!($id, _PIN), unsafe { concat_idents!(HAL_, $id, _GPIO_Port) }, concat_idents!(HAL_, $id, _Pin))?;
        }
    };
}

pub fn init_pins() -> Result<()> {
    init_pin(&LED_0_PIN, GPIOB, GPIO_PIN::GPIO_PIN_12, GpioMode::GPIO_MODE_INPUT);
    init_pin(&LED_1_PIN, GPIOB, GPIO_PIN::GPIO_PIN_13, GpioMode::GPIO_MODE_INPUT);
    init_pin(&LED_2_PIN, GPIOB, GPIO_PIN::GPIO_PIN_14, GpioMode::GPIO_MODE_INPUT);
    init_pin(&LED_3_PIN, GPIOB, GPIO_PIN::GPIO_PIN_15, GpioMode::GPIO_MODE_INPUT);

    Ok(())
}
