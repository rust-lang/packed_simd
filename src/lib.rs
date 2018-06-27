//! SPMD - Single Program Multiple Data
#![feature(rust_2018_preview, repr_simd, const_fn, platform_intrinsics,
           stdsimd, aarch64_target_feature, arm_target_feature)]
#![allow(non_camel_case_types, non_snake_case)]
#![cfg_attr(test, feature(plugin, hashmap_internals))]
#![cfg_attr(test, plugin(interpolate_idents))]
#![no_std]

#[macro_use]
extern crate cfg_if;

#[cfg(test)]
extern crate arrayvec;

use core::{cmp, default, fmt, hash, marker, mem, ops};

#[macro_use]
mod api;
mod codegen;
mod llvm;
mod masks;
mod sealed;

pub use self::masks::*;

/// SIMD vector type
///
/// # Examples
///
/// ```
/// # use spmd::Simd;
/// let v = Simd::<[i32; 4]>::new(0, 1, 2, 3);
/// assert_eq!(v.extract(2), 2);
/// ```
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Simd<A: sealed::SimdArray>(<A as sealed::SimdArray>::Tuple);

// 128-bit wide vector types

impl_i!([i8; 16]: i8x16 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 128-bit vector with 16 `i8` lanes.
);
impl_u!([u8; 16]: u8x16 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 128-bit vector with 16 `u8` lanes.
);
impl_m!([m8; 16]: m8x16 | i8 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 128-bit vector mask with 16 `m8` lanes.
);

impl_i!([i16; 8]: i16x8 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 128-bit vector with 8 `i16` lanes.
);
impl_u!([u16; 8]: u16x8 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 128-bit vector with 8 `u16` lanes.
);
impl_m!([m16; 8]: m16x8 | i16 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 128-bit vector mask with 8 `m16` lanes.
);

impl_i!([i32; 4]: i32x4 | x0, x1, x2, x3 |
        /// A 128-bit vector with 4 `i32` lanes.
);
impl_u!([u32; 4]: u32x4 | x0, x1, x2, x3 |
        /// A 128-bit vector with 4 `u32` lanes.
);
impl_f!([f32; 4]: f32x4 | x0, x1, x2, x3 |
        /// A 128-bit vector with 4 `f32` lanes.
);
impl_m!([m32; 4]: m32x4 | i32 | x0, x1, x2, x3 |
        /// A 128-bit vector mask with 4 `m32` lanes.
);

impl_i!([i64; 2]: i64x2 | x0, x1 |
        /// A 128-bit vector with 2 `i64` lanes.
);
impl_u!([u64; 2]: u64x2 | x0, x1 |
        /// A 128-bit vector with 2 `u64` lanes.
);
impl_f!([f64; 2]: f64x2 | x0, x1 |
        /// A 128-bit vector with 2 `f64` lanes.
);
impl_m!([m64; 2]: m64x2 | i64 | x0, x1 |
        /// A 128-bit vector mask with 2 `m64` lanes.
);

impl_i!([i128; 1]: i128x1 | x0 |
        /// A 128-bit vector with 1 `i128` lane.
);
impl_u!([u128; 1]: u128x1 | x0 |
        /// A 128-bit vector with 1 `u128` lane.
);
impl_m!([m128; 1]: m128x1 | i128 | x0 |
        /// A 128-bit vector mask with 1 `m128` lane.
);

// 256-bit wide vector types

impl_i!([i8; 32]: i8x32 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31 |
        /// A 256-bit vector with 32 `i8` lanes.
);
impl_u!([u8; 32]: u8x32 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31 |
        /// A 256-bit vector with 32 `u8` lanes.
);
impl_m!([m8; 32]: m8x32 | i8 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31 |
        /// A 256-bit vector mask with 32 `m8` lanes.
);

impl_i!([i16; 16]: i16x16 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 256-bit vector with 16 `i16` lanes.
);
impl_u!([u16; 16]: u16x16 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 256-bit vector with 16 `u16` lanes.
);
impl_m!([m16; 16]: m16x16 | i16 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 256-bit vector mask with 16 `m16` lanes.
);

impl_i!([i32; 8]: i32x8 | x0, x1, x2, x3, x4, x5, x6, x7  |
        /// A 256-bit vector with 8 `i32` lanes.
);
impl_u!([u32; 8]: u32x8 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 256-bit vector with 8 `u32` lanes.
);
impl_f!([f32; 8]: f32x8 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 256-bit vector with 8 `f32` lanes.
);
impl_m!([m32; 8]: m32x8 | i32 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 256-bit vector mask with 8 `m32` lanes.
);

impl_i!([i64; 4]: i64x4 | x0, x1, x2, x3 |
        /// A 256-bit vector with 4 `i64` lanes.
);
impl_u!([u64; 4]: u64x4 | x0, x1, x2, x3 |
        /// A 256-bit vector with 4 `u64` lanes.
);
impl_f!([f64; 4]: f64x4 | x0, x1, x2, x3 |
        /// A 256-bit vector with 4 `f64` lanes.
);
impl_m!([m64; 4]: m64x4 | i64 | x0, x1, x2, x3 |
        /// A 256-bit vector mask with 4 `m64` lanes.
);

impl_i!([i128; 2]: i128x2 | x0, x1 |
        /// A 256-bit vector with 2 `i128` lanes.
);
impl_u!([u128; 2]: u128x2 | x0, x1 |
        /// A 256-bit vector with 2 `u128` lanes.
);
impl_m!([m128; 2]: m128x2 | i128 | x0, x1 |
        /// A 256-bit vector mask with 2 `m128` lanes.
);
