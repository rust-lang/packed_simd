//! Sealed traits

use super::{m128, m16, m32, m64, m8};

/// Trait implemented by arrays that can be SIMD types
pub trait SimdArray {
    type Tuple: Copy + Clone;
    type T;
    const N: usize;
}

macro_rules! impl_simd_array {
    ([$elem_ty:ident; $elem_cnt:expr]: $tuple_id:ident | $($elem_tys:ident),*) => {
        #[derive(Copy, Clone)]
        #[repr(simd)]
        pub struct $tuple_id($(pub $elem_tys),*);

        impl SimdArray for [$elem_ty; $elem_cnt] {
            type Tuple = $tuple_id;
            type T = $elem_ty;
            const N: usize = $elem_cnt;
        }
    }
}

// 128-bit wide vectors

impl_simd_array!(
    [i8; 16]: i8x16 | i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8
);
impl_simd_array!(
    [u8; 16]: u8x16 | u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8
);
impl_simd_array!(
    [m8; 16]: m8x16 | i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8
);

impl_simd_array!([i16; 8]: i16x8 | i16, i16, i16, i16, i16, i16, i16, i16);
impl_simd_array!([u16; 8]: u16x8 | u16, u16, u16, u16, u16, u16, u16, u16);
impl_simd_array!([m16; 8]: m16x8 | i16, i16, i16, i16, i16, i16, i16, i16);

impl_simd_array!([i32; 4]: i32x4 | i32, i32, i32, i32);
impl_simd_array!([u32; 4]: u32x4 | u32, u32, u32, u32);
impl_simd_array!([f32; 4]: f32x4 | f32, f32, f32, f32);
impl_simd_array!([m32; 4]: m32x4 | i32, i32, i32, i32);

impl_simd_array!([i64; 2]: i64x2 | i64, i64);
impl_simd_array!([u64; 2]: u64x2 | u64, u64);
impl_simd_array!([f64; 2]: f64x2 | f64, f64);
impl_simd_array!([m64; 2]: m64x2 | i64, i64);

impl_simd_array!([i128; 1]: i128x1 | i128);
impl_simd_array!([u128; 1]: u128x1 | u128);
impl_simd_array!([m128; 1]: m128x1 | i128);

// 256-bit wide vectors

impl_simd_array!(
    [i8; 32]: i8x32 | i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8
);
impl_simd_array!(
    [u8; 32]: u8x32 | u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8
);
impl_simd_array!(
    [m8; 32]: m8x32 | i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8
);

impl_simd_array!(
    [i16; 16]: i16x16 | i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16
);
impl_simd_array!(
    [u16; 16]: u16x16 | u16,
    u16,
    u16,
    u16,
    u16,
    u16,
    u16,
    u16,
    u16,
    u16,
    u16,
    u16,
    u16,
    u16,
    u16,
    u16
);
impl_simd_array!(
    [m16; 16]: m16x16 | i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16
);

impl_simd_array!([i32; 8]: i32x8 | i32, i32, i32, i32, i32, i32, i32, i32);
impl_simd_array!([u32; 8]: u32x8 | u32, u32, u32, u32, u32, u32, u32, u32);
impl_simd_array!([f32; 8]: f32x8 | f32, f32, f32, f32, f32, f32, f32, f32);
impl_simd_array!([m32; 8]: m32x8 | i32, i32, i32, i32, i32, i32, i32, i32);

impl_simd_array!([i64; 4]: i64x4 | i64, i64, i64, i64);
impl_simd_array!([u64; 4]: u64x4 | u64, u64, u64, u64);
impl_simd_array!([f64; 4]: f64x4 | f64, f64, f64, f64);
impl_simd_array!([m64; 4]: m64x4 | i64, i64, i64, i64);

impl_simd_array!([i128; 2]: i128x2 | i128, i128);
impl_simd_array!([u128; 2]: u128x2 | u128, u128);
impl_simd_array!([m128; 2]: m128x2 | i128, i128);
