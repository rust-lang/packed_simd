//! SPMD - Single Program Multiple Data
#![feature(rust_2018_preview, repr_simd, const_fn, platform_intrinsics)]
#![allow(non_camel_case_types)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(interpolate_idents))]
#![no_std]

use core::{cmp, fmt, ops};

mod llvm;
mod sealed;
#[macro_use]
mod api;

/// SIMD vector type
///
/// # Examples
///
/// ```
/// # use spmd::Simd;
/// let v = Simd::<[i32; 4]>::new(0, 1, 2, 3);
/// assert_eq!(v.extract(2), 2);
/// ```
pub struct Simd<A: sealed::SimdArray>(<A as sealed::SimdArray>::Tuple);

impl_i!([i32; 4]: i32x4 | x0, x1, x2, x3 |
        /// A 128-bit vector with 4 `i32` lanes.
);
impl_u!([u32; 4]: u32x4 | x0, x1, x2, x3 |
        /// A 128-bit vector with 4 `u32` lanes.
);
impl_f!([f32; 4]: f32x4 | x0, x1, x2, x3 |
        /// A 128-bit vector with 4 `f32` lanes.
);
