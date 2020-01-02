use crate::hal::traits::Result;

pub trait UART {
    fn write(&self, s: &str) -> Result<()>;
}

// TODO replace with blanket impl
#[macro_export]
macro_rules! impl_uart_write {
    ($uart:ty) => {
        impl core::fmt::Write for $uart {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.write(s)
                    .map_err(|_| core::fmt::Error)
            }
        }
    };
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        {
            use crate::hal::debug_uart_get;
            use core::fmt::Write;
            let uart_local = debug_uart_get();
            let mut uart = uart_local.lock().unwrap();
            write!(*uart, $($arg)*).unwrap()
        }
    };
}

#[macro_export]
macro_rules! debug_println {
    () => (debug_print!("\n"));
    ($fmt:expr) => (debug_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (debug_print!(concat!($fmt, "\n"), $($arg)*));
}
