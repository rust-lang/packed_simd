#![feature(rust_2018_preview, use_extern_macros, avx512_target_feature, abi_vectorcall)]

#[cfg(test)]
extern crate packed_simd;

#[cfg(test)]
extern crate stdsimd_test;

#[cfg(test)]
mod api;
