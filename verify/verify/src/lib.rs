#![deny(warnings)]
#![feature(rust_2018_preview, avx512_target_feature, abi_vectorcall)]

#[cfg(test)]
extern crate packed_simd;

#[cfg(test)]
extern crate stdsimd_test;

#[cfg(test)]
mod api;
