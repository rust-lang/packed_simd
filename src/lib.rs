//! SPMD - Single Program Multiple Data
#![feature(
    rust_2018_preview, repr_simd, const_fn, platform_intrinsics, stdsimd,
    aarch64_target_feature, arm_target_feature
)]
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
mod sealed;

/// Packed SIMD vector type
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

mod masks;
pub use self::masks::*;

mod v128;
pub use self::v128::*;

mod v256;
pub use self::v256::*;
