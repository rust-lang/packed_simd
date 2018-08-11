//! aobench: Ambient Occlusion Renderer benchmark.
//!
//! Based on [aobench](https://code.google.com/archive/p/aobench/) by Syoyo
//! Fujita.
#![deny(warnings)]
#![allow(non_snake_case, non_camel_case_types)]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        many_single_char_names,
        similar_names,
        cast_precision_loss,
        inline_always,
        cast_possible_truncation,
        cast_sign_loss,
        identity_op
    )
)]

#[macro_use]
extern crate cfg_if;
extern crate failure;
extern crate packed_simd;
extern crate png;
extern crate rayon;

pub mod ambient_occlusion;
pub mod geometry;
pub mod image;
pub mod intersection;
pub mod random;
pub mod scene;

pub mod scalar;
pub mod scalar_parallel;
pub mod vector;
pub mod vector_parallel;

pub use self::image::Image;
pub use self::scene::Scene;
