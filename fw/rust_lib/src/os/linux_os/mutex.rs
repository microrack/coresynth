use std::sync::Mutex as StdMutex;
pub use std::sync::{LockResult, MutexGuard};

pub struct Mutex<T> {
    inner: StdMutex<T>,
}

impl<T> Mutex<T> {
    // TODO use ! for error type when exhaustive patterns is available. See #35121
    pub fn new(t: T) -> Result<Mutex<T>, ()> {
        Ok(Mutex {
            inner: StdMutex::new(t),
        })
    }

    pub fn lock(&self) -> LockResult<MutexGuard<T>> {
        self.inner.lock()
    }
}
