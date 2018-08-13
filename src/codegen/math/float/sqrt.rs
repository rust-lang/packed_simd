//! Vertical floating-point `sqrt`
#![allow(unused)]

// FIXME 64-bit sqrtgle elem vectors missqrtg

use crate::*;

crate trait Sqrt {
    fn sqrt(self) -> Self;
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.sqrt.v2f32"]
    fn sqrt_v2f32(x: f32x2) -> f32x2;
    #[link_name = "llvm.sqrt.v4f32"]
    fn sqrt_v4f32(x: f32x4) -> f32x4;
    #[link_name = "llvm.sqrt.v8f32"]
    fn sqrt_v8f32(x: f32x8) -> f32x8;
    #[link_name = "llvm.sqrt.v16f32"]
    fn sqrt_v16f32(x: f32x16) -> f32x16;
    /* FIXME 64-bit sqrtgle elem vectors
    #[link_name = "llvm.sqrt.v1f64"]
    fn sqrt_v1f64(x: f64x1) -> f64x1;
     */
    #[link_name = "llvm.sqrt.v2f64"]
    fn sqrt_v2f64(x: f64x2) -> f64x2;
    #[link_name = "llvm.sqrt.v4f64"]
    fn sqrt_v4f64(x: f64x4) -> f64x4;
    #[link_name = "llvm.sqrt.v8f64"]
    fn sqrt_v8f64(x: f64x8) -> f64x8;

    #[link_name = "llvm.sqrt.f32"]
    fn sqrt_f32(x: f32) -> f32;
    #[link_name = "llvm.sqrt.f64"]
    fn sqrt_f64(x: f64) -> f64;
}

macro_rules! impl_vsqrt {
    ($vid:ident: $llvm_fn:ident) => {
        impl Sqrt for $vid {
            #[inline]
            fn sqrt(self) -> Self {
                unsafe { mem::transmute($llvm_fn(mem::transmute(self))) }
            }
        }
    };
}

macro_rules! impl_ssqrt {
    ($vid:ident => [$sid:ident; $scount:expr]: $llvm_fn:ident) => {
        impl Sqrt for $vid {
            #[inline]
            fn sqrt(self) -> Self {
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
        impl_ssqrt!(f32x2 => [f32; 2]: sqrt_f32);
        impl_ssqrt!(f32x4 => [f32; 4]: sqrt_f32);
        impl_ssqrt!(f32x8 => [f32; 8]: sqrt_f32);
        impl_ssqrt!(f32x16 => [f32; 16]: sqrt_f32);

        impl_ssqrt!(f64x2 => [f64; 2]: sqrt_f64);
        impl_ssqrt!(f64x4 => [f64; 4]: sqrt_f64);
        impl_ssqrt!(f64x8 => [f64; 8]: sqrt_f64);
    } else if #[cfg(all(target_arch = "x86_64", feature = "sleef-sys"))] {
        use ::sleef_sys::*;
        impl_ssqrt!(f32x2 => [f32; 2]: sqrt_f32);
        impl_vsqrt!(f32x16: sqrt_v16f32);
        impl_vsqrt!(f64x8: sqrt_v8f64);
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                impl_vsqrt!(f32x4: Sleef_sqrtf4_avx2128);
                impl_vsqrt!(f32x8: Sleef_sqrtf8_avx2);
                impl_vsqrt!(f64x2: Sleef_sqrtd2_avx2128);
                impl_vsqrt!(f64x4: Sleef_sqrtd4_avx2);
            } else if #[cfg(target_feature = "avx")] {
                impl_vsqrt!(f32x4: Sleef_sqrtf4_sse4);
                impl_vsqrt!(f32x8: Sleef_sqrtf8_avx);
                impl_vsqrt!(f64x2: Sleef_sqrtd2_sse4);
                impl_vsqrt!(f64x4: Sleef_sqrtd4_avx);
            } else if #[cfg(target_feature = "sse4.2")] {
                impl_vsqrt!(f32x4: Sleef_sqrtf4_sse4);
                impl_vsqrt!(f32x8: sqrt_v8f32);
                impl_vsqrt!(f64x2: Sleef_sqrtd2_sse4);
                impl_vsqrt!(f64x4: sqrt_v4f64);
            } else {
                impl_vsqrt!(f32x4: sqrt_v4f32);
                impl_vsqrt!(f32x8: sqrt_v8f32);
                impl_vsqrt!(f64x2: sqrt_v2f64);
                impl_vsqrt!(f64x4: sqrt_v4f64);
            }
        }
    } else {
        impl_ssqrt!(f32x2 => [f32; 2]: sqrt_f32);
        impl_vsqrt!(f32x4: sqrt_v4f32);
        impl_vsqrt!(f32x8: sqrt_v8f32);
        impl_vsqrt!(f32x16: sqrt_v16f32);

        impl_vsqrt!(f64x2: sqrt_v2f64);
        impl_vsqrt!(f64x4: sqrt_v4f64);
        impl_vsqrt!(f64x8: sqrt_v8f64);
    }
}
