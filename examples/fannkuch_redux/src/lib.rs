//! Fannkuch redux
#![deny(warnings, rust_2018_idioms)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(
    clippy::similar_names,
    clippy::many_single_char_names,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::must_use_candidate
)]

pub mod scalar;
pub mod simd;

pub fn fannkuch_redux(n: usize, alg: usize) -> (i32, i32) {
    match alg {
        0 => simd::fannkuch_redux(n),
        1 => scalar::fannkuch_redux(n),
        v => panic!("unknown algorithm value: {}", v),
    }
}
