//! `FromTrunc` and `IntoTrunc` implementations for portable 512-bit wide vectors
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::*;

impl_from_trunc!(i8x64: u8x64, m8x64);
impl_from_trunc!(u8x64: i8x64, m8x64);
impl_from_trunc_mask!(m8x64: i8x64, u8x64);

impl_from_trunc!(i16x32: i8x32, u8x32, m8x32, u16x32, m16x32);
impl_from_trunc!(u16x32: i8x32, u8x32, m8x32, i16x32, m16x32);
impl_from_trunc_mask!(m16x32: i8x32, u8x32, m8x32, i16x32, u16x32);

impl_from_trunc!(
    i32x16: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, u32x16, f32x16, m32x16
);
impl_from_trunc!(
    u32x16: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, f32x16, m32x16
);
impl_from_trunc!(
    f32x16: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, u32x16, m32x16
);
impl_from_trunc_mask!(
    m32x16: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, u32x16, f32x16
);

impl_from_trunc!(
    i64x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8, m32x8,
        u64x8, f64x8, m64x8
);
impl_from_trunc!(
    u64x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8, m32x8,
        i64x8, f64x8, m64x8
);
impl_from_trunc!(
    f64x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8, m32x8,
        i64x8, u64x8, m64x8
);
impl_from_trunc_mask!(
    m64x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8, m32x8,
        i64x8, u64x8, f64x8
);
