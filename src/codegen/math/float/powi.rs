//! Vertical floating-point `powf`
#![allow(unused)]

use crate::*;

crate trait Powi {
    fn powi(self, x: i32) -> Self;
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.powi.v2f32"]
    fn powi_v2f32(x: f32x2, y: i32) -> f32x2;
    #[link_name = "llvm.powi.v4f32"]
    fn powi_v4f32(x: f32x4, y: i32) -> f32x4;
    #[link_name = "llvm.powi.v8f32"]
    fn powi_v8f32(x: f32x8, y: i32) -> f32x8;
    #[link_name = "llvm.powi.v16f32"]
    fn powi_v16f32(x: f32x16, y: i32) -> f32x16;
    /* FIXME 64-bit powigle elem vectors
    #[link_name = "llvm.powi.v1f64"]
    fn powi_v1f64(x: f64x1, y: i32) -> f64x1;
     */
    #[link_name = "llvm.powi.v2f64"]
    fn powi_v2f64(x: f64x2, y: i32) -> f64x2;
    #[link_name = "llvm.powi.v4f64"]
    fn powi_v4f64(x: f64x4, y: i32) -> f64x4;
    #[link_name = "llvm.powi.v8f64"]
    fn powi_v8f64(x: f64x8, y: i32) -> f64x8;

    #[link_name = "llvm.powi.f32"]
    fn powi_f32(x: f32, y: i32) -> f32;
    #[link_name = "llvm.powi.f64"]
    fn powi_f64(x: f64, y: i32) -> f64;
}

macro_rules! impl_ {
    ($id:ident, $fn_id:ident) => {
        impl Powi for $id {
            fn powi(self, x: i32) -> Self {
                use mem::transmute;
                unsafe { transmute($fn_id(transmute(self), x)) }
            }
        }
    };
}

impl_!(f32x2, powi_v2f32);
impl_!(f32x4, powi_v4f32);
impl_!(f32x8, powi_v8f32);
impl_!(f32x16, powi_v16f32);
impl_!(f64x2, powi_v2f64);
impl_!(f64x4, powi_v4f64);
impl_!(f64x8, powi_v8f64);
