//! Sealed traits

use super::m32;

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

impl_simd_array!([i32; 4]: i32x4 | i32, i32, i32, i32);
impl_simd_array!([u32; 4]: u32x4 | u32, u32, u32, u32);
impl_simd_array!([f32; 4]: f32x4 | f32, f32, f32, f32);
impl_simd_array!([m32; 4]: m32x4 | i32, i32, i32, i32);
