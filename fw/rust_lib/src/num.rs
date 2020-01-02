// See https://github.com/rust-lang/rust/issues/50145

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(all(target_arch="arm", target_os="none"))] {
        pub trait Round {
            fn round(self) -> Self;
        }

        impl Round for f32 {
            fn round(self) -> f32 {
                // micromath does not provide round
                // but LLVM intrinsic does not require libm
                unsafe { core::intrinsics::roundf32(self) }
            }
        }
    }
}
