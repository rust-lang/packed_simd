//! Vertical floating-point `acos`
#![allow(unused)]

// FIXME 64-bit 1 elem vectors acos

use crate::*;

crate trait Acos {
    fn acos(self) -> Self;
}

macro_rules! define_acos {
    ($name:ident, $basetype:ty, $simdtype:ty, $lanes:expr, $trait:path) => {
        fn $name(x: $simdtype) -> $simdtype {
            use core::intrinsics::transmute;
            let mut buf: [$basetype; $lanes] = unsafe { transmute(x) };
            for elem in &mut buf {
                *elem = <$basetype as $trait>::acos(*elem);
            }
            unsafe { transmute(buf) }
        }
    };

    (f32 => $name:ident, $type:ty, $lanes:expr) => {
        define_acos!($name, f32, $type, $lanes, libm::F32Ext);
    };

    (f64 => $name:ident, $type:ty, $lanes:expr) => {
        define_acos!($name, f64, $type, $lanes, libm::F64Ext);
    };
}

// llvm does not seem to expose the hyperbolic versions of trigonometric
// functions; we thus call the classical rust versions on all of them (which
// stem from cmath).
define_acos!(f32 => acos_v2f32, f32x2, 2);
define_acos!(f32 => acos_v4f32, f32x4, 4);
define_acos!(f32 => acos_v8f32, f32x8, 8);
define_acos!(f32 => acos_v16f32, f32x16, 16);

define_acos!(f64 => acos_v2f64, f64x2, 2);
define_acos!(f64 => acos_v4f64, f64x4, 4);
define_acos!(f64 => acos_v8f64, f64x8, 8);

fn acos_f32(x: f32) -> f32 {
    libm::F32Ext::acos(x)
}

fn acos_f64(x: f64) -> f64 {
    libm::F64Ext::acos(x)
}

gen_unary_impl_table!(Acos, acos);

cfg_if! {
    if #[cfg(target_arch = "s390x")] {
        // FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/14
        impl_unary!(f32x2[f32; 2]: acos_f32);
        impl_unary!(f32x4[f32; 4]: acos_f32);
        impl_unary!(f32x8[f32; 8]: acos_f32);
        impl_unary!(f32x16[f32; 16]: acos_f32);

        impl_unary!(f64x2[f64; 2]: acos_f64);
        impl_unary!(f64x4[f64; 4]: acos_f64);
        impl_unary!(f64x8[f64; 8]: acos_f64);
    } else if #[cfg(all(target_arch = "x86_64", feature = "sleef-sys"))] {
        use sleef_sys::*;
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                impl_unary!(f32x2[t => f32x4]: Sleef_acosf4_u10avx2128);
                impl_unary!(f32x16[h => f32x8]: Sleef_acosf8_u10avx2);
                impl_unary!(f64x8[h => f64x4]: Sleef_acosd4_u10avx2);

                impl_unary!(f32x4: Sleef_acosf4_u10avx2128);
                impl_unary!(f32x8: Sleef_acosf8_u10avx2);
                impl_unary!(f64x2: Sleef_acosd2_u10avx2128);
                impl_unary!(f64x4: Sleef_acosd4_u10avx2);
            } else if #[cfg(target_feature = "avx")] {
                impl_unary!(f32x2[t => f32x4]: Sleef_acosf4_u10sse4);
                impl_unary!(f32x16[h => f32x8]: Sleef_acosf8_u10avx);
                impl_unary!(f64x8[h => f64x4]: Sleef_acosd4_u10avx);

                impl_unary!(f32x4: Sleef_acosf4_u10sse4);
                impl_unary!(f32x8: Sleef_acosf8_u10avx);
                impl_unary!(f64x2: Sleef_acosd2_u10sse4);
                impl_unary!(f64x4: Sleef_acosd4_u10avx);
            } else if #[cfg(target_feature = "sse4.2")] {
                impl_unary!(f32x2[t => f32x4]: Sleef_acosf4_u10sse4);
                impl_unary!(f32x16[q => f32x4]: Sleef_acosf4_u10sse4);
                impl_unary!(f64x8[q => f64x2]: Sleef_acosd2_u10sse4);

                impl_unary!(f32x4: Sleef_acosf4_u10sse4);
                impl_unary!(f32x8[h => f32x4]: Sleef_acosf4_u10sse4);
                impl_unary!(f64x2: Sleef_acosd2_u10sse4);
                impl_unary!(f64x4[h => f64x2]: Sleef_acosd2_u10sse4);
            } else {
                impl_unary!(f32x2[f32; 2]: acos_f32);
                impl_unary!(f32x16: acos_v16f32);
                impl_unary!(f64x8: acos_v8f64);

                impl_unary!(f32x4: acos_v4f32);
                impl_unary!(f32x8: acos_v8f32);
                impl_unary!(f64x2: acos_v2f64);
                impl_unary!(f64x4: acos_v4f64);
            }
        }
    } else {
        impl_unary!(f32x2[f32; 2]: acos_f32);
        impl_unary!(f32x4: acos_v4f32);
        impl_unary!(f32x8: acos_v8f32);
        impl_unary!(f32x16: acos_v16f32);

        impl_unary!(f64x2: acos_v2f64);
        impl_unary!(f64x4: acos_v4f64);
        impl_unary!(f64x8: acos_v8f64);
    }
}
