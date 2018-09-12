//! Mask reductions implementation for `x86` and `x86_64` targets with `AVX`

/// x86/x86_64 256-bit AVX implementation
#[cfg(target_feature = "avx")]
macro_rules! x86_m8x32_avx_impl {
    ($id:ident) => {
        impl All for $id {
            #[inline]
            #[target_feature(enable = "avx")]
            unsafe fn all(self) -> bool {
                #[cfg(target_arch = "x86")]
                use ::arch::x86::_mm256_testc_si256;
                #[cfg(target_arch = "x86_64")]
                use ::arch::x86_64::_mm256_testc_si256;
                _mm256_testc_si256(
                    ::mem::transmute(self),
                    ::mem::transmute($id::splat(true)),
                ) != 0
            }
        }
        impl Any for $id {
            #[inline]
            #[target_feature(enable = "avx")]
            unsafe fn any(self) -> bool {
                #[cfg(target_arch = "x86")]
                use ::arch::x86::_mm256_testz_si256;
                #[cfg(target_arch = "x86_64")]
                use ::arch::x86_64::_mm256_testz_si256;
                _mm256_testz_si256(
                    ::mem::transmute(self),
                    ::mem::transmute(self),
                ) == 0
            }
        }
    };
}
