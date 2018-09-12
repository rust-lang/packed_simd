#![deny(warnings)]
#![feature(rust_2018_preview, avx512_target_feature, abi_vectorcall)]

#[cfg(test)]
extern crate packed_simd;

#[cfg(test)]
extern crate stdsimd_test;

#[cfg(test)]
#[macro_use]
extern crate cfg_if;

#[cfg(test)]
mod api;
