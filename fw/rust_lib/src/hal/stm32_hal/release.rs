use crate::hal::traits::Result;
use crate::os::Mutex;
use crate::peripheral::Static;

pub trait Release<P> {
    fn checked_release(&self, ptr: *mut P) -> Result<()>;
}

pub fn checked_release<P, T:Release<P>>(statics:&[&Static<Mutex<T>>], ptr:*mut P) -> Result<()> {
    for statik in statics {
        unsafe {
            statik.unsafe_get().checked_release(ptr)?;
        }
    }
    Ok(())
}
