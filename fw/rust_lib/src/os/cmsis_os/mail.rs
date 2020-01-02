use super::bindings::*;
use super::{Duration, Error, Result};

use crate::ctypes::c_void;
use core::marker::PhantomData;

// Deriving clone is safe until drop is implemented
#[derive(Clone)]
pub struct MailSender<T> {
    id: osMailQId,
    __phantom__: PhantomData<T>,
}

unsafe impl<T: Send> Send for MailSender<T> {}
unsafe impl<T: Send> Sync for MailSender<T> {}

pub struct MailReceiver<T> {
    id: osMailQId,
    __phantom__: PhantomData<T>,
}

impl<T> MailSender<T> {
    pub fn send(&self, item: T) -> Result<()> {
        let item_ptr = unsafe {
            let res = osMailAlloc(self.id, osWaitForever) as *mut T;
            if res.is_null() {
                return Err(Error {
                    call: "osMailAlloc",
                    status: None,
                });
            }
            res.write_unaligned(item);
            res
        };

        let status = unsafe { osMailPut(self.id, item_ptr as *mut c_void) };
        return match status {
            osStatus::osOK => Ok(()),
            _ => Err(Error {
                call: "osMailPut",
                status: Some(status),
            }),
        };
    }
}

// TODO merge with linux os one
#[derive(Debug)]
pub enum RecvTimeoutError {
    Timeout,
    Other,
}

impl<T> MailReceiver<T> {
    pub fn recv(&self) -> Result<T> {
        return self.recv_impl(osWaitForever);
    }

    // TODO more error info
    pub fn recv_timeout(&self, timeout: Duration) -> core::result::Result<T, RecvTimeoutError> {
        use osStatus::{osOK, osEventTimeout};
        use RecvTimeoutError::*;

        return self.recv_impl(timeout.as_ms())
            .map_err(|e| match e.call {
                "osMailGet" => match e.status {
                    // osMailGet can return osOK when there's no message in 2 cases
                    // 1. It was called from ISR. In this case osMailGet will ignore timeout
                    // 2. Is was called with timeout of 0
                    Some(osEventTimeout) | Some(osOK) => Timeout,
                    _ => Other,
                },
                _ => Other,
            });
    }

    fn recv_impl(&self, timeout_ms: u32) -> Result<T> {
        let event = unsafe { osMailGet(self.id, timeout_ms) };
        if event.status != osStatus::osEventMail {
            return Err(Error {
                call: "osMailGet",
                status: Some(event.status),
            });
        }

        let void_ptr = unsafe {
            *event.value.p.as_ref()
        };

        let item = unsafe {
            let item_ptr = void_ptr as *mut T;
            item_ptr.read_unaligned()
        };

        let status = unsafe { osMailFree(self.id, void_ptr) };
        return match status {
            osStatus::osOK => Ok(item),
            _ => Err(Error {
                call: "osMailFree",
                status: Some(status),
            }),
        };
    }
}

pub fn mail_queue<T>(size: usize) -> Result<(MailSender<T>, MailReceiver<T>)> {
    // TODO use proper cast
    assert!(size <= core::u32::MAX as usize);
    let size = size as u32;

    let item_size = core::mem::size_of::<T>();
    // TODO use proper cast
    assert!(item_size <= core::u32::MAX as usize);
    let item_size = item_size as u32;

    // TODO can move theese? Can drop theese?
    let mut mail_queue_cb : *mut os_mailQ_cb = core::ptr::null_mut();
    let mail_queue_def = osMailQDef_t {
        queue_sz: size,
        item_sz: item_size,
        cb: &mut mail_queue_cb,
    };

    let mail_queue_id = unsafe { osMailCreate(&mail_queue_def, 0) };

    // mail_queue_id is actually a pointer
    if mail_queue_id == 0 {
        return Err(Error {
            call: "osMailCreate",
            status: None,
        });
    }
    let sender = MailSender {
        id: mail_queue_id,
        __phantom__: PhantomData,
    };
    let receiver = MailReceiver {
        id: mail_queue_id,
        __phantom__: PhantomData,
    };

    return Ok((sender, receiver));
}
