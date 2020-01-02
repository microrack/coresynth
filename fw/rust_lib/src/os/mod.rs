#[cfg(feature = "cmsis-os")]
pub mod cmsis_os;
#[cfg(feature = "cmsis-os")]
pub use self::cmsis_os::*;

#[cfg(feature = "linux-os")]
pub mod linux_os;
#[cfg(feature = "linux-os")]
pub use self::linux_os::*;

#[cfg(not(any(feature = "linux-os", feature = "cmsis-os")))]
compile_error!("You must select one os feature");

mod closure;
pub use self::closure::*;

mod timers;
pub use self::timers::*;