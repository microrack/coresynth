#[derive(Default)]
#[repr(align(16))]
struct ClosureBuffer([u8; 16]);

pub struct Closure {
    trait_object: core::raw::TraitObject,
    data: ClosureBuffer,
    data_drop: fn(data: &mut ClosureBuffer) -> (),
}

// CustomClosure is Send because actual closure in ::new is required to Send
unsafe impl Send for Closure {}

impl core::fmt::Debug for Closure {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        // TODO more verbose
        "CustomClosure".fmt(f)
    }
}

impl Closure {
    fn data_drop<F>(data: &mut ClosureBuffer) {
        let f = &mut data.0 as *mut [u8;16] as *mut F;
        unsafe { core::ptr::drop_in_place::<F>(f) };
    }

    // TODO allow FnOnce
    pub fn new<F: FnMut() + Send>(f: F) -> Closure {
        assert!(core::mem::size_of::<F>() <= core::mem::size_of::<ClosureBuffer>());
        assert!(core::mem::align_of::<F>() <= core::mem::align_of::<ClosureBuffer>());

        let fn_ref = &f;
        let fn_to = fn_ref as &dyn FnMut();
        let trait_object: core::raw::TraitObject = unsafe { core::mem::transmute(fn_to) };

        let mut data = ClosureBuffer::default();
        {
            let data_ptr = &mut data.0 as *mut [u8;16] as *mut F;
            unsafe {
                data_ptr.write_unaligned(f);
            }
        }

        Closure {
            trait_object,
            data,
            data_drop: Self::data_drop::<F>,
        }
    }

    pub fn call(&mut self) {
        let mut trait_object = self.trait_object;
        trait_object.data = &mut self.data as *const _ as *mut _;
        let fn_to : &mut dyn FnMut() = unsafe { core::mem::transmute(trait_object) };
        (fn_to)();
    }
}

impl Drop for Closure {
    fn drop(&mut self) {
        (self.data_drop)(&mut self.data);
    }
}
