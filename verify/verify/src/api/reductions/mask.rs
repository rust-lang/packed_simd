//! Verify the mask reduction API.

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        cfg_if! {
            if #[cfg(target_feature = "sse2")] {
                mod sse2;
            }
        }
    }
}
