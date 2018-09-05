//! Implements different algorithms for summing a slice of `f32`s

use super::f32s;
use rayon::prelude::*;
use std::{mem, slice};

pub fn scalar(x: &[f32]) -> f32 {
    x.iter().sum()
}

pub fn vector(x: &[f32]) -> f32 {
    assert_eq!(x.len() % f32s::lanes(), 0);

    let mut sum = f32s::splat(0.);
    for i in (0..x.len()).step_by(f32s::lanes()) {
        sum += f32s::from_slice_unaligned(&x[i..]);
    }
    sum.sum()
}

pub fn vector_par(x: &[f32]) -> f32 {
    let len: usize = x.len();
    assert_eq!(len % 8, 0);

    // find the first properly aligned element
    let (i, _): (usize, _) = x
        .iter()
        .enumerate()
        .find(|&(_, y): &(usize, &f32)| {
            (y as *const f32) as usize % mem::align_of::<f32s>() == 0
        }).unwrap();

    let (head, tail) = x.split_at(i);
    let head_sum: f32 = head.iter().sum();

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
    let tail: &[f32s] = unsafe {
        slice::from_raw_parts(
            tail.as_ptr() as *const f32s,
            tail.len() / f32s::lanes(),
        )
    };
    let tail_sum: f32s = tail.into_par_iter().sum();
    head_sum + tail_sum.sum()
}

pub fn fastest(x: &[f32]) -> f64 {
    let mut sum = 0_f64;
    for &x in x {
        sum += f64::from(x);
    }
    sum
}
