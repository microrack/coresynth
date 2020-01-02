pub use super::bindings::osSemaphoreId;
use super::bindings::*;
use super::{Error, Result};

pub struct Semaphore {
    id: osSemaphoreId,
}

#[allow(dead_code)]
impl Semaphore {
    pub fn new(count: u32) -> Result<Semaphore> {
        assert!(count <= core::i32::MAX as u32);
        let semaphore_def = osSemaphoreDef_t { dummy: 0 };
        let semaphore_id = unsafe { osSemaphoreCreate(&semaphore_def, count as i32) };
        return match semaphore_id {
            // semaphore_id is actually a pointer
            0 => Err(Error {
                call: "osSemaphoreCreate",
                status: None,
            }),
            _ => Ok(Semaphore { id: semaphore_id }),
        };
    }

    // TODO it's a copy-paste, move to common code
    pub fn empty(count: u32) -> Result<Semaphore> {
        assert!(count <= core::i32::MAX as u32);
        let res = Semaphore::new(count)?;
        for _ in 0..count {
            // semaphore created full
            res.acquire()?;
        }
        return Ok(res);
    }

    pub fn acquire(&self) -> Result<()> {
        let result = unsafe { osSemaphoreWait(self.id, osWaitForever) };
        return match result {
            -1 => Err(Error {
                call: "osSemaphoreWait",
                status: None,
            }),
            _ => Ok(()),
        };
    }

    pub fn release(&self) -> Result<()> {
        let status = unsafe { osSemaphoreRelease(self.id) };
        return match status {
            osStatus::osOK => Ok(()),
            _ => Err(Error {
                call: "osSemaphoreRelease",
                status: Some(status),
            }),
        };
    }

    // TODO implement drop compatible with this
    pub fn get_id(&self) -> osSemaphoreId {
        self.id
    }

    pub fn from_id(id: osSemaphoreId) -> Semaphore {
        Semaphore { id }
    }
}

unsafe impl Sync for Semaphore {}
