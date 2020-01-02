use core::cell::UnsafeCell;

pub struct Static<T> {
    cell: UnsafeCell<Option<T>>,
}

impl<T> Static<T> {
    pub const fn new() -> Static<T> {
        Static {
            cell: UnsafeCell::new(None)
        }
    }

    pub fn init(&self, t:T) {
        // TODO maybe should run in interrupt-free context
        let opt_ptr = self.cell.get();
        {
            let opt = unsafe { opt_ptr.as_ref() };
            let opt = match opt {
                Some(res) => res,
                None => panic!("Static cell is null")
            };
            if opt.is_some() {
                panic!("Static already initialized");
            }
        }
        unsafe { *opt_ptr = Some(t) };
    }

    pub fn get(&self) -> Option<&T> {
        // TODO check for thread-safety
        let opt_ptr = self.cell.get();
        let opt = unsafe { opt_ptr.as_ref() };
        let opt = match opt {
            Some(res) => res,
            None => panic!("Static cell is null")
        };

        return match opt {
            &Some(ref res) => Some(&res),
            &None => None,
        }
    }
}

impl<T> core::ops::Deref for Static<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.get().unwrap()
    }
}

unsafe impl<T: Send> Send for Static<T> {}
unsafe impl<T: Sync> Sync for Static<T> {}
