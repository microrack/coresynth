use core::fmt;

use crate::os::{Mutex, Semaphore};

use super::bindings::{
    UART_HandleTypeDef,
    HAL_UART_Transmit_IT,
    HAL_StatusTypeDef,
    huart1
};

use super::release::{checked_release, Release};

use crate::peripheral::{Static};

use crate::hal::uart_printf::{UART};
use crate::hal::traits::{Error, Result};

#[allow(non_camel_case_types)]
pub struct STM32_UART {
    uart: *mut UART_HandleTypeDef,
    semaphore: Semaphore,
}

impl STM32_UART {
    fn new(uart: *mut UART_HandleTypeDef) -> Result<STM32_UART> {
        Ok(STM32_UART {
            uart,
            semaphore: Semaphore::empty(1).map_err(|_| Error {
                call: "UART::new Semaphore::empty",
            })?,
        })
    }

    fn write_slice_unsafe(&self, bytes:&[u8]) -> Result<()> {
        let len = bytes.len();
        if len > core::u16::MAX as usize {
            panic!("Can't send more than 65kB at once");
        }
        let status = unsafe { HAL_UART_Transmit_IT(
            self.uart,
            // for some reason, HAL_UART_Transmit_IT accept buffer as mutable
            bytes.as_ptr() as *mut u8,
            len as u16)
        };
        match status {
            HAL_StatusTypeDef::HAL_OK => Ok(()),
            _ => Err(Error { call: "UART::write HAL_UART_Transmit_IT" })
        }
    }

    fn write_slice_blocking(&self, bytes: &[u8]) -> Result<()> {
        self.write_slice_unsafe(bytes)?;
        self.semaphore.acquire().map_err(|_| Error {
            call: "UART::write Semaphore::acquire",
        })?;
        Ok(())
    }
}

impl UART for STM32_UART {
    fn write(&self, s: &str) -> Result<()> {
        let bytes = s.as_bytes();
        for chunk in bytes.chunks(core::u16::MAX as usize) {
            self.write_slice_blocking(chunk)?;
        }
        Ok(())
    }
}

impl Release<UART_HandleTypeDef> for STM32_UART {
    fn checked_release(&self, ptr:*mut UART_HandleTypeDef) -> Result<()> {
        if self.uart == ptr {
            self.semaphore.release().map_err(|_| Error {
                call: "UART::checked_release Semaphore::release",
            })?;
        }
        Ok(())
    }
}

impl_uart_write!(STM32_UART);

pub static DEBUG_UART: Static<Mutex<STM32_UART>> = Static::new();
pub static ALL_UARTS: [&Static<Mutex<STM32_UART>>; 1] = [&DEBUG_UART];

pub fn debug_uart_init_static() -> Result<()> {
    let uart = STM32_UART::new(unsafe { &mut huart1 })?; // TODO specify uart
    let mutex = Mutex::new(uart)
        .map_err(|_| Error {
            call: "UART init static Mutex::new",
        })?;

    DEBUG_UART.init(mutex);
    Ok(())
}

pub fn debug_uart_get() -> &'static Mutex<STM32_UART> {
    &DEBUG_UART
}

#[no_mangle]
pub extern "C" fn HAL_UART_TxCpltCallback(huart: *mut UART_HandleTypeDef) {
    checked_release(&ALL_UARTS, huart)
        .expect("HAL_UART_TxCpltCallback checked_release");
}
