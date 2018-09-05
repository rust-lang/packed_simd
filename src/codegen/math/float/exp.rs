//! Vertical floating-point `exp`
#![allow(unused)]

// FIXME 64-bit expgle elem vectors misexpg

use crate::*;

crate trait Exp {
    fn exp(self) -> Self;
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.exp.v2f32"]
    fn exp_v2f32(x: f32x2) -> f32x2;
    #[link_name = "llvm.exp.v4f32"]
    fn exp_v4f32(x: f32x4) -> f32x4;
    #[link_name = "llvm.exp.v8f32"]
    fn exp_v8f32(x: f32x8) -> f32x8;
    #[link_name = "llvm.exp.v16f32"]
    fn exp_v16f32(x: f32x16) -> f32x16;
    /* FIXME 64-bit expgle elem vectors
    #[link_name = "llvm.exp.v1f64"]
    fn exp_v1f64(x: f64x1) -> f64x1;
     */
    #[link_name = "llvm.exp.v2f64"]
    fn exp_v2f64(x: f64x2) -> f64x2;
    #[link_name = "llvm.exp.v4f64"]
    fn exp_v4f64(x: f64x4) -> f64x4;
    #[link_name = "llvm.exp.v8f64"]
    fn exp_v8f64(x: f64x8) -> f64x8;

    #[link_name = "llvm.exp.f32"]
    fn exp_f32(x: f32) -> f32;
    #[link_name = "llvm.exp.f64"]
    fn exp_f64(x: f64) -> f64;
}

macro_rules! impl_vexp {
    ($vid:ident: $llvm_fn:ident) => {
        impl Exp for $vid {
            #[inline]
            fn exp(self) -> Self {
                unsafe { mem::transmute($llvm_fn(mem::transmute(self))) }
            }
        }
    };
}

macro_rules! impl_sexp {
    ($vid:ident => [$sid:ident; $scount:expr]: $llvm_fn:ident) => {
        impl Exp for $vid {
            #[inline]
            fn exp(self) -> Self {
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
        impl_sexp!(f32x2 => [f32; 2]: exp_f32);
        impl_sexp!(f32x4 => [f32; 4]: exp_f32);
        impl_sexp!(f32x8 => [f32; 8]: exp_f32);
        impl_sexp!(f32x16 => [f32; 16]: exp_f32);

        impl_sexp!(f64x2 => [f64; 2]: exp_f64);
        impl_sexp!(f64x4 => [f64; 4]: exp_f64);
        impl_sexp!(f64x8 => [f64; 8]: exp_f64);
    } else if #[cfg(all(target_arch = "x86_64", feature = "sleef-sys"))] {
        use ::sleef_sys::*;
        impl_sexp!(f32x2 => [f32; 2]: exp_f32);
        impl_vexp!(f32x16: exp_v16f32);
        impl_vexp!(f64x8: exp_v8f64);
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                impl_vexp!(f32x4: Sleef_expf4_u10avx2128);
                impl_vexp!(f32x8: Sleef_expf8_u10avx2);
                impl_vexp!(f64x2: Sleef_expd2_u10avx2128);
                impl_vexp!(f64x4: Sleef_expd4_u10avx2);
            } else if #[cfg(target_feature = "avx")] {
                impl_vexp!(f32x4: Sleef_expf4_u10sse4);
                impl_vexp!(f32x8: Sleef_expf8_u10avx);
                impl_vexp!(f64x2: Sleef_expd2_u10sse4);
                impl_vexp!(f64x4: Sleef_expd4_u10avx);
            } else if #[cfg(target_feature = "sse4.2")] {
                impl_vexp!(f32x4: Sleef_expf4_sse4);
                impl_vexp!(f32x8: exp_v8f32);
                impl_vexp!(f64x2: Sleef_expd2_sse4);
                impl_vexp!(f64x4: exp_v4f64);
            } else if #[cfg(target_feature = "sse2")] {
                impl_vexp!(f32x4: Sleef_expf4_sse2);
                impl_vexp!(f32x8: exp_v8f32);
                impl_vexp!(f64x2: Sleef_expd2_sse2);
                impl_vexp!(f64x4: exp_v4f64);
            } else {
                impl_vexp!(f32x4: exp_v4f32);
                impl_vexp!(f32x8: exp_v8f32);
                impl_vexp!(f64x2: exp_v2f64);
                impl_vexp!(f64x4: exp_v4f64);
            }
        }
    } else {
        impl_sexp!(f32x2 => [f32; 2]: exp_f32);
        impl_vexp!(f32x4: exp_v4f32);
        impl_vexp!(f32x8: exp_v8f32);
        impl_vexp!(f32x16: exp_v16f32);

        impl_vexp!(f64x2: exp_v2f64);
        impl_vexp!(f64x4: exp_v4f64);
        impl_vexp!(f64x8: exp_v8f64);
    }
}
