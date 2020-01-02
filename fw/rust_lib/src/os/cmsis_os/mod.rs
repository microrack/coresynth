mod bindings;

mod mail;
pub use self::mail::*;

mod mutex;
pub use self::mutex::*;

mod semaphore;
pub use self::semaphore::*;

mod task;
pub use self::task::*;

mod time;
pub use self::time::*;

#[derive(Debug)]
pub struct Error {
    call: &'static str,
    status: Option<bindings::osStatus>,
}

pub type Result<T> = core::result::Result<T, Error>;
