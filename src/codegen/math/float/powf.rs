//! Vertical floating-point `powf`
#![allow(unused)]

// FIXME 64-bit powfgle elem vectors mispowfg

use crate::*;

crate trait Powf {
    fn powf(self, x: Self) -> Self;
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.pow.v2f32"]
    fn powf_v2f32(x: f32x2, y: f32x2) -> f32x2;
    #[link_name = "llvm.pow.v4f32"]
    fn powf_v4f32(x: f32x4, y: f32x4) -> f32x4;
    #[link_name = "llvm.pow.v8f32"]
    fn powf_v8f32(x: f32x8, y: f32x8) -> f32x8;
    #[link_name = "llvm.pow.v16f32"]
    fn powf_v16f32(x: f32x16, y: f32x16) -> f32x16;
    /* FIXME 64-bit powfgle elem vectors
    #[link_name = "llvm.pow.v1f64"]
    fn powf_v1f64(x: f64x1, y: f64x1) -> f64x1;
     */
    #[link_name = "llvm.pow.v2f64"]
    fn powf_v2f64(x: f64x2, y: f64x2) -> f64x2;
    #[link_name = "llvm.pow.v4f64"]
    fn powf_v4f64(x: f64x4, y: f64x4) -> f64x4;
    #[link_name = "llvm.pow.v8f64"]
    fn powf_v8f64(x: f64x8, y: f64x8) -> f64x8;

    #[link_name = "llvm.pow.f32"]
    fn powf_f32(x: f32, y: f32) -> f32;
    #[link_name = "llvm.pow.f64"]
    fn powf_f64(x: f64, y: f64) -> f64;
}

macro_rules! impl_vpowf {
    ($vid:ident: $llvm_fn:ident) => {
        impl Powf for $vid {
            #[inline]
            fn powf(self, e: Self) -> Self {
                unsafe {
                    mem::transmute($llvm_fn(
                        mem::transmute(self),
                        mem::transmute(e),
                    ))
                }
            }
        }
    };
}

macro_rules! impl_spowf {
    ($vid:ident => [$sid:ident; $scount:expr]: $llvm_fn:ident) => {
        impl Powf for $vid {
            #[inline]
            fn powf(self, e: Self) -> Self {
                unsafe {
                    let mut scalars: [$sid; $scount] = mem::transmute(self);
                    let exponents: [$sid; $scount] = mem::transmute(e);
                    for i in 0..$scount {
                        let s = *scalars.get_unchecked(i);
                        let e = *exponents.get_unchecked(i);
                        *scalars.get_unchecked_mut(i) = $llvm_fn(s, e);
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
        impl_spowf!(f32x2 => [f32; 2]: powf_f32);
        impl_spowf!(f32x4 => [f32; 4]: powf_f32);
        impl_spowf!(f32x8 => [f32; 8]: powf_f32);
        impl_spowf!(f32x16 => [f32; 16]: powf_f32);

        impl_spowf!(f64x2 => [f64; 2]: powf_f64);
        impl_spowf!(f64x4 => [f64; 4]: powf_f64);
        impl_spowf!(f64x8 => [f64; 8]: powf_f64);
    } else if #[cfg(all(target_arch = "x86_64", feature = "sleef-sys"))] {
        use ::sleef_sys::*;
        impl_spowf!(f32x2 => [f32; 2]: powf_f32);
        impl_vpowf!(f32x16: powf_v16f32);
        impl_vpowf!(f64x8: powf_v8f64);
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                impl_vpowf!(f32x4: Sleef_powf4_u10avx2128);
                impl_vpowf!(f32x8: Sleef_powf8_u10avx2);
                impl_vpowf!(f64x2: Sleef_powd2_u10avx2128);
                impl_vpowf!(f64x4: Sleef_powd4_u10avx2);
            } else if #[cfg(target_feature = "avx")] {
                impl_vpowf!(f32x4: Sleef_powf4_u10sse4);
                impl_vpowf!(f32x8: Sleef_powf8_u10avx);
                impl_vpowf!(f64x2: Sleef_powd2_u10sse4);
                impl_vpowf!(f64x4: Sleef_powd4_u10avx);
            } else if #[cfg(target_feature = "sse4.2")] {
                impl_vpowf!(f32x4: Sleef_powf4_u10sse4);
                impl_vpowf!(f32x8: powf_v8f32);
                impl_vpowf!(f64x2: Sleef_powd2_u10sse4);
                impl_vpowf!(f64x4: powf_v4f64);
            } else if #[cfg(target_feature = "sse2")] {
                impl_vpowf!(f32x4: Sleef_powf4_u10sse2);
                impl_vpowf!(f32x8: powf_v8f32);
                impl_vpowf!(f64x2: Sleef_powd2_u10sse2);
                impl_vpowf!(f64x4: powf_v4f64);
            } else {
                impl_vpowf!(f32x4: powf_v4f32);
                impl_vpowf!(f32x8: powf_v8f32);
                impl_vpowf!(f64x2: powf_v2f64);
                impl_vpowf!(f64x4: powf_v4f64);
            }
        }
    } else {
        impl_spowf!(f32x2 => [f32; 2]: powf_f32);
        impl_vpowf!(f32x4: powf_v4f32);
        impl_vpowf!(f32x8: powf_v8f32);
        impl_vpowf!(f32x16: powf_v16f32);

        impl_vpowf!(f64x2: powf_v2f64);
        impl_vpowf!(f64x4: powf_v4f64);
        impl_vpowf!(f64x8: powf_v8f64);
    }
}
