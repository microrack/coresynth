#[macro_use]
pub mod uart_printf;

#[cfg(feature = "stm32-hal")]
pub mod stm32_hal;
#[cfg(feature = "stm32-hal")]
pub use self::stm32_hal::*;

#[cfg(feature = "stub-hal")]
pub mod stub_hal;
#[cfg(feature = "stub-hal")]
pub use self::stub_hal::*;

#[cfg(not(any(feature = "stm32-hal", feature = "stub-hal")))]
compile_error!("You must select one hal feature");

pub mod traits {
    #[derive(Debug)]
    pub struct Error {
        pub call: &'static str,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum PinState {
        Reset,
        Set,
    }

    pub trait Pin {
        fn write(&mut self, state: PinState);
        fn toggle(&mut self);
        fn read(&self) -> PinState;
    }

    pub trait SPI {
        fn write(&self, bytes: &[u8]) -> Result<()>;
        fn read(&self, bytes: &mut [u8]) -> Result<()>;
        fn exchange(&self, tx_bytes: &[u8], rx_bytes: &mut [u8]) -> Result<()>;
    }
}
