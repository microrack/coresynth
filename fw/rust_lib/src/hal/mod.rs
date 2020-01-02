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

    #[allow(non_camel_case_types)]
    pub enum GpioMode {
        GPIO_MODE_INPUT              = 0x00000000,   /* Input Floating Mode                   */
        GPIO_MODE_OUTPUT_PP          = 0x00000001,   /* Output Push Pull Mode                 */
        GPIO_MODE_OUTPUT_OD          = 0x00000011,   /* Output Open Drain Mode                */
        GPIO_MODE_AF_PP              = 0x00000002,   /* Alternate Function Push Pull Mode     */
        GPIO_MODE_AF_OD              = 0x00000012,   /* Alternate Function Open Drain Mode    */
        GPIO_MODE_ANALOG             = 0x00000003,   /* Analog Mode  */  
        GPIO_MODE_IT_RISING          = 0x10110000,   /* External Interrupt Mode with Rising edge trigger detection          */
        GPIO_MODE_IT_FALLING         = 0x10210000,   /* External Interrupt Mode with Falling edge trigger detection         */
        GPIO_MODE_IT_RISING_FALLING  = 0x10310000,   /* External Interrupt Mode with Rising/Falling edge trigger detection  */
        GPIO_MODE_EVT_RISING         = 0x10120000,   /* External Event Mode with Rising edge trigger detection               */
        GPIO_MODE_EVT_FALLING        = 0x10220000,   /* External Event Mode with Falling edge trigger detection              */
        GPIO_MODE_EVT_RISING_FALLING = 0x10320000,   /* External Event Mode with Rising/Falling edge trigger detection       */
    }

    pub trait Pin {
        fn write(&mut self, state: PinState);
        fn toggle(&mut self);
        fn read(&self) -> PinState;
        fn mode(&self, mode: GpioMode);
    }

    pub trait SPI {
        fn write(&self, bytes: &[u8]) -> Result<()>;
        fn read(&self, bytes: &mut [u8]) -> Result<()>;
        fn exchange(&self, tx_bytes: &[u8], rx_bytes: &mut [u8]) -> Result<()>;
    }
}
