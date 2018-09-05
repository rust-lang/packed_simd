//! Vertical floating-point `sqrt`
#![allow(unused)]

use llvm::simd_fsqrt;

// FIXME 64-bit 1 elem vectors sqrte

use crate::*;

crate trait Sqrte {
    fn sqrte(self) -> Self;
}

macro_rules! impl_vsqrt {
    ($vid:ident: $llvm_fn:ident) => {
        impl Sqrte for $vid {
            #[inline]
            fn sqrte(self) -> Self {
                unsafe { mem::transmute($llvm_fn(self.0)) }
            }
        }
    };
}

macro_rules! impl_vsqrt_ {
    ($vid:ident: $llvm_fn:ident) => {
        impl Sqrte for $vid {
            #[inline]
            fn sqrte(self) -> Self {
                unsafe { mem::transmute($llvm_fn(mem::transmute(self))) }
            }
        }
    };
}

cfg_if! {
    if #[cfg(all(target_arch = "x86_64", feature = "sleef-sys"))] {
        use ::sleef_sys::*;
        impl_vsqrt!(f32x2: simd_fsqrt);
        impl_vsqrt!(f32x16: simd_fsqrt);
        impl_vsqrt!(f64x8: simd_fsqrt);
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                impl_vsqrt_!(f32x4: Sleef_sqrtf4_u35avx2128);
                impl_vsqrt_!(f32x8: Sleef_sqrtf8_u35avx2);
                impl_vsqrt_!(f64x2: Sleef_sqrtd2_u35avx2128);
                impl_vsqrt_!(f64x4: Sleef_sqrtd4_u35avx2);
            } else if #[cfg(target_feature = "avx")] {
                impl_vsqrt_!(f32x4: Sleef_sqrtf4_u35sse4);
                impl_vsqrt_!(f32x8: Sleef_sqrtf8_u35avx);
                impl_vsqrt_!(f64x2: Sleef_sqrtd2_u35sse4);
                impl_vsqrt_!(f64x4: Sleef_sqrtd4_u35avx);
            } else if #[cfg(target_feature = "sse4.2")] {
                impl_vsqrt_!(f32x4: Sleef_sqrtf4_u35sse4);
                impl_vsqrt!(f32x8: simd_fsqrt);
                impl_vsqrt_!(f64x2: Sleef_sqrtd2_u35sse4);
                impl_vsqrt!(f64x4: simd_fsqrt);
            } else {
                impl_vsqrt!(f32x4: simd_fsqrt);
                impl_vsqrt!(f32x8: simd_fsqrt);
                impl_vsqrt!(f64x2: simd_fsqrt);
                impl_vsqrt!(f64x4: simd_fsqrt);
            }
        }
    } else {
        impl_vsqrt!(f32x2: simd_fsqrt);
        impl_vsqrt!(f32x4: simd_fsqrt);
        impl_vsqrt!(f32x8: simd_fsqrt);
        impl_vsqrt!(f32x16: simd_fsqrt);

        impl_vsqrt!(f64x2: simd_fsqrt);
        impl_vsqrt!(f64x4: simd_fsqrt);
        impl_vsqrt!(f64x8: simd_fsqrt);
    }
}
