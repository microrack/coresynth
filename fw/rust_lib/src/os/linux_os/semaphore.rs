use std::sync::Condvar;
use std::sync::Mutex as StdMutex;

#[allow(dead_code)]
pub struct Semaphore {
    count: u32,
    free: StdMutex<u32>,
    condvar: Condvar,
}

#[allow(dead_code)]
impl Semaphore {
    // TODO use ! for error type when exhaustive patterns is available. See #35121
    pub fn new(count: u32) -> Result<Semaphore, ()> {
        Ok(Semaphore {
            count,
            free: StdMutex::new(count),
            condvar: Condvar::new(),
        })
    }

    // TODO it's a copy-paste, move to common code
    // TODO use ! for error type when exhaustive patterns is available. See #35121
    pub fn empty(count: u32) -> Result<Semaphore, ()> {
        let res = Semaphore::new(count)?;
        for _ in 0..count {
            // semaphore created full
            res.acquire()?;
        }
        return Ok(res);
    }

    pub fn acquire(&self) -> Result<(), ()> {
        let mut lock = self.free.lock().unwrap();
        while *lock == 0 {
            lock = self.condvar.wait(lock).unwrap();
        }
        *lock -= 1;
        Ok(())
    }

    pub fn release(&self) -> Result<(), ()> {
        let mut lock = self.free.lock().unwrap();
        if *lock == self.count {
            // All tokens already released
            return Err(());
        }
        *lock += 1;
        self.condvar.notify_one();
        Ok(())
    }
}
