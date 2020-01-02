use super::bindings::*;
use super::{Duration, Error, Result};

use crate::ctypes::c_void;

extern "C" fn thread_trampoline(argument: *const c_void) {
    let thread_fn: extern "C" fn() = unsafe { core::mem::transmute(argument) };
    thread_fn();

    let current_thread_id = unsafe { osThreadGetId() };
    if current_thread_id == 0 {
        panic!("osThreadGetId failed in thread_trampoline");
    }
    let terminate_status = unsafe { osThreadTerminate(current_thread_id) };
    if terminate_status != osStatus::osOK {
        panic!("osThreadTerminate failed in thread_trampoline");
    }
}

const MAX_C_STR_LEN: usize = 64;
type CStr = [u8; MAX_C_STR_LEN];

fn make_c_str(s: &str) -> core::result::Result<CStr, ()> {
    let len = s.len();
    if len > MAX_C_STR_LEN - 1 {
        // str is too large
        return Err(());
    }

    let mut result = [0u8; MAX_C_STR_LEN];
    result[..len].copy_from_slice(s.as_bytes());
    result[len] = 0;

    return Ok(result);
}

pub fn spawn(name: &str, stack_size: u32, thread_fn: extern "C" fn()) -> Result<()> {
    let name_cstr = make_c_str(name).map_err(|_| Error {
        call: "make_c_str",
        status: None,
    })?;
    let name_ptr = name_cstr.as_ptr() as *mut crate::ctypes::c_char;

    let thread_def = osThreadDef_t {
        name: name_ptr,
        pthread: Some(thread_trampoline),
        tpriority: osPriority::osPriorityNormal,
        instances: 1,
        stacksize: stack_size,
    };

    let arg = thread_fn as *mut c_void;

    let thread_id = unsafe { osThreadCreate(&thread_def, arg) };
    return match thread_id {
        // thread_id is actually a pointer
        0 => Err(Error {
            call: "osThreadCreate",
            status: None,
        }),
        _ => Ok(()),
    };
}

pub fn delay(delay: Duration) -> Result<()> {
    let status = unsafe { osDelay(delay.as_ms()) };
    return match status {
        osStatus::osOK => Ok(()),
        _ => Err(Error {
            call: "osDelay",
            status: Some(status),
        }),
    };
}
