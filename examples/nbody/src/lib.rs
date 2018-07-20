//! The N-body benchmark from the [benchmarks game][bg].
//!
//! [bg]: https://benchmarksgame.alioth.debian.org/u64q/nbody-description.
//! html#nbody
#![deny(warnings)]

extern crate packed_simd;

pub mod scalar;
pub mod simd;

pub fn run(n: usize) -> (f64, f64) {
    simd::run(n)
}

#[cfg(test)]
const RESULTS: &[(usize, &str, &str)] = &[
    (50_000_000_usize, "-0.169075164", "-0.169059907")
];
