#![deny(warnings)]
#![cfg_attr(test, feature(plugin, rust_2018_preview, avx512_target_feature, abi_vectorcall))]
#![cfg_attr(test, plugin(interpolate_idents))]

#[cfg(test)]
extern crate packed_simd;

#[cfg(test)]
extern crate stdsimd_test;

#[cfg(test)]
#[macro_use]
extern crate cfg_if;

#[cfg(test)]
mod api;

#[cfg(test)]
cfg_if! {
    if #[cfg(debug_assertions)] {
        compile_error!("the verify tests only run in --release mode");
    }
}
