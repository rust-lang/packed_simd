//! Sim<[T; N]> - Packed vector type
#![feature(
    rust_2018_preview, repr_simd, const_fn, platform_intrinsics, stdsimd,
    aarch64_target_feature, arm_target_feature, link_llvm_intrinsics,
    core_intrinsics
)]
#![allow(non_camel_case_types, non_snake_case)]
#![cfg_attr(test, feature(plugin, hashmap_internals))]
#![cfg_attr(test, plugin(interpolate_idents))]
#![no_std]

#[macro_use]
extern crate cfg_if;

#[cfg(test)]
extern crate arrayvec;

#[allow(unused_imports)]
use core::{cmp, default, f32, f64, fmt, hash, intrinsics, marker, mem, ops};

#[macro_use]
mod api;
mod codegen;
mod llvm;
mod sealed;

/// Packed vector type
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
