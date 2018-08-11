//! Fannkuch redux
#![deny(warnings)]
#![allow(non_snake_case, non_camel_case_types)]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        similar_names,
        many_single_char_names,
        cast_possible_truncation,
        cast_sign_loss,
        cast_possible_wrap
    )
)]
extern crate packed_simd;

pub mod scalar;
pub mod simd;

pub fn fannkuch_redux(n: usize, alg: usize) -> (i32, i32) {
    match alg {
        0 => simd::fannkuch_redux(n),
        1 => scalar::fannkuch_redux(n),
        v => panic!("unknown algorithm value: {}", v),
    }
}
