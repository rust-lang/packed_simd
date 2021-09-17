//! Vertical floating-point `tanh`
#![allow(unused)]

// FIXME 64-bit 1 elem vectors tanh

use crate::*;

crate trait Tanh {
    fn tanh(self) -> Self;
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.tan.v2f32"]
    fn tan_v2f32(x: f32x2) -> f32x2;
    #[link_name = "llvm.tan.v4f32"]
    fn tanh_v4f32(x: f32x4) -> f32x4;
    #[link_name = "llvm.tan.v8f32"]
    fn tanh_v8f32(x: f32x8) -> f32x8;
    #[link_name = "llvm.tan.v16f32"]
    fn tanh_v16f32(x: f32x16) -> f32x16;
    /* FIXME 64-bit single elem vectors
    #[link_name = "llvm.tan.v1f64"]
    fn tan_v1f64(x: f64x1) -> f64x1;
     */
    #[link_name = "llvm.tan.v2f64"]
    fn tanh_v2f64(x: f64x2) -> f64x2;
    #[link_name = "llvm.tan.v4f64"]
    fn tanh_v4f64(x: f64x4) -> f64x4;
    #[link_name = "llvm.tan.v8f64"]
    fn tanh_v8f64(x: f64x8) -> f64x8;

    #[link_name = "llvm.tan.f32"]
    fn tanh_f32(x: f32) -> f32;
    #[link_name = "llvm.tan.f64"]
    fn tanh_f64(x: f64) -> f64;
}

gen_unary_impl_table!(Tanh, tanh);

cfg_if! {
    if #[cfg(target_arch = "s390x")] {
        // FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/14
        impl_unary!(f32x2[f32; 2]: tanh_f32);
        impl_unary!(f32x4[f32; 4]: tanh_f32);
        impl_unary!(f32x8[f32; 8]: tanh_f32);
        impl_unary!(f32x16[f32; 16]: tanh_f32);

        impl_unary!(f64x2[f64; 2]: tanh_f64);
        impl_unary!(f64x4[f64; 4]: tanh_f64);
        impl_unary!(f64x8[f64; 8]: tanh_f64);
    } else if #[cfg(all(target_arch = "x86_64", feature = "sleef-sys"))] {
        use sleef_sys::*;
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                impl_unary!(f32x2[t => f32x4]: Sleef_tanhf4_u10avx2128);
                impl_unary!(f32x16[h => f32x8]: Sleef_tanhf8_u10avx2);
                impl_unary!(f64x8[h => f64x4]: Sleef_tanhd4_u10avx2);

                impl_unary!(f32x4: Sleef_tanhf4_u10avx2128);
                impl_unary!(f32x8: Sleef_tanhf8_u10avx2);
                impl_unary!(f64x2: Sleef_tanhd2_u10avx2128);
                impl_unary!(f64x4: Sleef_tanhd4_u10avx2);
            } else if #[cfg(target_feature = "avx")] {
                impl_unary!(f32x2[t => f32x4]: Sleef_tanhf4_u10sse4);
                impl_unary!(f32x16[h => f32x8]: Sleef_tanhf8_u10avx);
                impl_unary!(f64x8[h => f64x4]: Sleef_tanhd4_u10avx);

                impl_unary!(f32x4: Sleef_tanhf4_u10sse4);
                impl_unary!(f32x8: Sleef_tanhf8_u10avx);
                impl_unary!(f64x2: Sleef_tanhd2_u10sse4);
                impl_unary!(f64x4: Sleef_tanhd4_u10avx);
            } else if #[cfg(target_feature = "sse4.2")] {
                impl_unary!(f32x2[t => f32x4]: Sleef_tanhf4_u10sse4);
                impl_unary!(f32x16[q => f32x4]: Sleef_tanhf4_u10sse4);
                impl_unary!(f64x8[q => f64x2]: Sleef_tanhd2_u10sse4);

                impl_unary!(f32x4: Sleef_tanhf4_u10sse4);
                impl_unary!(f32x8[h => f32x4]: Sleef_tanhf4_u10sse4);
                impl_unary!(f64x2: Sleef_tanhd2_u10sse4);
                impl_unary!(f64x4[h => f64x2]: Sleef_tanhd2_u10sse4);
            } else {
                impl_unary!(f32x2[f32; 2]: tanh_f32);
                impl_unary!(f32x16: tanh_v16f32);
                impl_unary!(f64x8: tanh_v8f64);

                impl_unary!(f32x4: tanh_v4f32);
                impl_unary!(f32x8: tanh_v8f32);
                impl_unary!(f64x2: tanh_v2f64);
                impl_unary!(f64x4: tanh_v4f64);
            }
        }
    } else {
        impl_unary!(f32x2[f32; 2]: tanh_f32);
        impl_unary!(f32x4: tanh_v4f32);
        impl_unary!(f32x8: tanh_v8f32);
        impl_unary!(f32x16: tanh_v16f32);

        impl_unary!(f64x2: tanh_v2f64);
        impl_unary!(f64x4: tanh_v4f64);
        impl_unary!(f64x8: tanh_v8f64);
    }
}
