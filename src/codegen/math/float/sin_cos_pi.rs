//! Vertical floating-point `sin_cos`
#![allow(unused)]

// FIXME 64-bit 1 elem vectors sin_cos

use crate::*;

crate trait SinCosPi: Sized {
    fn sin_cos_pi(self) -> (Self, Self);
}

macro_rules! impl_vsin_cos {
    ($vid:ident: $llvm_fn:ident) => {
        impl SinCosPi for $vid {
            #[inline]
            fn sin_cos_pi(self) -> (Self, Self) {
                unsafe { mem::transmute($llvm_fn(mem::transmute(self))) }
            }
        }
    };
}

macro_rules! impl_def {
    ($vid:ident, $PI:path) => {
        impl SinCosPi for $vid {
            #[inline]
            fn sin_cos_pi(self) -> (Self, Self) {
                let v = self * Self::splat($PI);
                (v.sin(), v.cos())
            }
        }
    };
}

cfg_if! {
    if #[cfg(all(target_arch = "x86_64", feature = "sleef-sys"))] {
        use ::sleef_sys::*;
        impl_def!(f32x2, core::f32::consts::PI);
        impl_def!(f32x16, core::f32::consts::PI);
        impl_def!(f64x8, core::f64::consts::PI);
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                impl_vsin_cos!(f32x4: Sleef_sincospif4_u05avx2128);
                impl_vsin_cos!(f32x8: Sleef_sincospif8_u05avx2);
                impl_vsin_cos!(f64x2: Sleef_sincospid2_u05avx2128);
                impl_vsin_cos!(f64x4: Sleef_sincospid4_u05avx2);
            } else if #[cfg(target_feature = "avx")] {
                impl_vsin_cos!(f32x4: Sleef_sincospif4_u05sse4);
                impl_vsin_cos!(f32x8: Sleef_sincospif8_u05avx);
                impl_vsin_cos!(f64x2: Sleef_sincospid2_u05sse4);
                impl_vsin_cos!(f64x4: Sleef_sincospid4_u05avx);
            } else if #[cfg(target_feature = "sse4.2")] {
                impl_vsin_cos!(f32x4: Sleef_sincospif4_u05sse4);
                impl_def!(f32x8, core::f32::consts::PI);
                impl_vsin_cos!(f64x2: Sleef_sincospid2_u05sse4);
                impl_def!(f64x4, core::f64::consts::PI);
            } else {
                impl_def!(f32x4, core::f32::consts::PI);
                impl_def!(f32x8, core::f32::consts::PI);
                impl_def!(f64x2, core::f64::consts::PI);
                impl_def!(f64x4, core::f64::consts::PI);
            }
        }
    } else {
        impl_def!(f32x2, core::f32::consts::PI);
        impl_def!(f32x4, core::f32::consts::PI);
        impl_def!(f32x8, core::f32::consts::PI);
        impl_def!(f32x16, core::f32::consts::PI);

        impl_def!(f64x2, core::f64::consts::PI);
        impl_def!(f64x4, core::f64::consts::PI);
        impl_def!(f64x8, core::f64::consts::PI);
    }
}
