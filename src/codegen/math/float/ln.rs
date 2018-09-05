//! Vertical floating-point `ln`
#![allow(unused)]

// FIXME 64-bit lngle elem vectors mislng

use crate::*;

crate trait Ln {
    fn ln(self) -> Self;
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.log.v2f32"]
    fn ln_v2f32(x: f32x2) -> f32x2;
    #[link_name = "llvm.log.v4f32"]
    fn ln_v4f32(x: f32x4) -> f32x4;
    #[link_name = "llvm.log.v8f32"]
    fn ln_v8f32(x: f32x8) -> f32x8;
    #[link_name = "llvm.log.v16f32"]
    fn ln_v16f32(x: f32x16) -> f32x16;
    /* FIXME 64-bit lngle elem vectors
    #[link_name = "llvm.log.v1f64"]
    fn ln_v1f64(x: f64x1) -> f64x1;
     */
    #[link_name = "llvm.log.v2f64"]
    fn ln_v2f64(x: f64x2) -> f64x2;
    #[link_name = "llvm.log.v4f64"]
    fn ln_v4f64(x: f64x4) -> f64x4;
    #[link_name = "llvm.log.v8f64"]
    fn ln_v8f64(x: f64x8) -> f64x8;

    #[link_name = "llvm.log.f32"]
    fn ln_f32(x: f32) -> f32;
    #[link_name = "llvm.log.f64"]
    fn ln_f64(x: f64) -> f64;
}

macro_rules! impl_vln {
    ($vid:ident: $llvm_fn:ident) => {
        impl Ln for $vid {
            #[inline]
            fn ln(self) -> Self {
                unsafe { mem::transmute($llvm_fn(mem::transmute(self))) }
            }
        }
    };
}

macro_rules! impl_sln {
    ($vid:ident => [$sid:ident; $scount:expr]: $llvm_fn:ident) => {
        impl Ln for $vid {
            #[inline]
            fn ln(self) -> Self {
                unsafe {
                    let mut scalars: [$sid; $scount] = mem::transmute(self);
                    for i in &mut scalars {
                        *i = $llvm_fn(*i);
                    }
                    mem::transmute(scalars)
                }
            }
        }
    };
}

cfg_if! {
    if #[cfg(target_arch = "s390x")] {
        // FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/14
        impl_sln!(f32x2 => [f32; 2]: ln_f32);
        impl_sln!(f32x4 => [f32; 4]: ln_f32);
        impl_sln!(f32x8 => [f32; 8]: ln_f32);
        impl_sln!(f32x16 => [f32; 16]: ln_f32);

        impl_sln!(f64x2 => [f64; 2]: ln_f64);
        impl_sln!(f64x4 => [f64; 4]: ln_f64);
        impl_sln!(f64x8 => [f64; 8]: ln_f64);
    } else if #[cfg(all(target_arch = "x86_64", feature = "sleef-sys"))] {
        use ::sleef_sys::*;
        impl_sln!(f32x2 => [f32; 2]: ln_f32);
        impl_vln!(f32x16: ln_v16f32);
        impl_vln!(f64x8: ln_v8f64);
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                impl_vln!(f32x4: Sleef_logf4_u10avx2128);
                impl_vln!(f32x8: Sleef_logf8_u10avx2);
                impl_vln!(f64x2: Sleef_logd2_u10avx2128);
                impl_vln!(f64x4: Sleef_logd4_u10avx2);
            } else if #[cfg(target_feature = "avx")] {
                impl_vln!(f32x4: Sleef_logf4_u10sse4);
                impl_vln!(f32x8: Sleef_logf8_u10avx);
                impl_vln!(f64x2: Sleef_logd2_u10sse4);
                impl_vln!(f64x4: Sleef_logd4_u10avx);
            } else if #[cfg(target_feature = "sse4.2")] {
                impl_vln!(f32x4: Sleef_logf4_sse4);
                impl_vln!(f32x8: ln_v8f32);
                impl_vln!(f64x2: Sleef_logd2_sse4);
                impl_vln!(f64x4: ln_v4f64);
            } else if #[cfg(target_feature = "sse2")] {
                impl_vln!(f32x4: Sleef_logf4_sse2);
                impl_vln!(f32x8: ln_v8f32);
                impl_vln!(f64x2: Sleef_logd2_sse2);
                impl_vln!(f64x4: ln_v4f64);
            } else {
                impl_vln!(f32x4: ln_v4f32);
                impl_vln!(f32x8: ln_v8f32);
                impl_vln!(f64x2: ln_v2f64);
                impl_vln!(f64x4: ln_v4f64);
            }
        }
    } else {
        impl_sln!(f32x2 => [f32; 2]: ln_f32);
        impl_vln!(f32x4: ln_v4f32);
        impl_vln!(f32x8: ln_v8f32);
        impl_vln!(f32x16: ln_v16f32);

        impl_vln!(f64x2: ln_v2f64);
        impl_vln!(f64x4: ln_v4f64);
        impl_vln!(f64x8: ln_v8f64);
    }
}
