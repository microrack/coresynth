use super::Duration;
use std::thread::{sleep, Builder as ThreadBuilder};

pub fn spawn(name: &str, stack_size: u32, thread_fn: extern "C" fn()) -> Result<(), ()> {
    ThreadBuilder::new()
        .name(name.to_string())
        .stack_size(stack_size as usize)
        .spawn(move || thread_fn())
        .map(|_| ())
        .map_err(|_| ())
}

// TODO use ! for error type when exhaustive patterns is available. See #35121
pub fn delay(delay: Duration) -> Result<(), ()> {
    sleep(delay.into());
    Ok(())
}
