use super::bindings::*;
use super::{Error, Result};

use core::cell::UnsafeCell;
use core::ops::Deref;
use core::ops::DerefMut;

struct InnerMutex {
    id: osMutexId,
}

impl InnerMutex {
    pub fn new() -> Result<InnerMutex> {
        let mutex_def = osMutexDef_t { dummy: 0 };
        let mutex_id = unsafe { osMutexCreate(&mutex_def) };
        return match mutex_id {
            // mutex_id is actually a pointer
            0 => Err(Error {
                call: "osMutexCreate",
                status: None,
            }),
            _ => Ok(InnerMutex { id: mutex_id }),
        };
    }

    pub fn lock(&self) -> Result<()> {
        let status = unsafe { osMutexWait(self.id, osWaitForever) };
        return match status {
            osStatus::osOK => Ok(()),
            _ => Err(Error {
                call: "osMutexWait",
                status: Some(status),
            }),
        };
    }

    pub fn unlock(&self) -> Result<()> {
        let status = unsafe { osMutexRelease(self.id) };
        return match status {
            osStatus::osOK => Ok(()),
            _ => Err(Error {
                call: "osMutexRelease",
                status: Some(status),
            }),
        };
    }
}

#[must_use]
pub struct MutexGuard<'a, T: 'a> {
    // funny underscores due to how Deref/DerefMut currently work (they
    // disregard field privacy).
    __lock: &'a Mutex<T>,
}

impl<'mutex, T> MutexGuard<'mutex, T> {
    fn new(lock: &'mutex Mutex<T>) -> MutexGuard<'mutex, T> {
        MutexGuard { __lock: lock }
    }
}

impl<'mutex, T> Deref for MutexGuard<'mutex, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.__lock.data.get() }
    }
}

impl<'mutex, T> DerefMut for MutexGuard<'mutex, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.__lock.data.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        // No way to handle error during drop
        self.__lock.inner.unlock().unwrap();
    }
}

pub struct Mutex<T> {
    inner: InnerMutex,
    data: UnsafeCell<T>,
}

#[allow(dead_code)]
impl<T> Mutex<T> {
    pub fn new(t: T) -> Result<Mutex<T>> {
        Ok(Mutex {
            inner: InnerMutex::new()?,
            data: UnsafeCell::new(t),
        })
    }

    pub fn lock(&self) -> Result<MutexGuard<T>> {
        self.inner.lock()?;
        Ok(MutexGuard::new(self))
    }

    pub unsafe fn unsafe_get(&self) -> &T {
        &*self.data.get()
    }
}

unsafe impl<T> Sync for Mutex<T> {}
