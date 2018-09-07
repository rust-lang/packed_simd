//! The mandelbrot benchmark from the [benchmarks game][bg].
//!
//! [bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/mandelbrot.html#mandelbrot

#![deny(warnings)]
#![cfg_attr(feature = "cargo-clippy", feature(tool_lints))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )
)]

extern crate packed_simd;
extern crate rayon;
extern crate time;
#[cfg(feature = "ispc")]
#[macro_use]
extern crate ispc;
use std::io;

pub mod output;
pub use output::{Format, FormatFn};

#[cfg(feature = "ispc")]
pub mod ispc_;
pub mod par_simd;
pub mod scalar;
pub mod simd;

pub const LIMIT: u32 = 50;

pub struct Mandelbrot {
    // output image width/height:
    pub width: usize,
    pub height: usize,
    // region:
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
    // output format
    format: Format,
}

impl Mandelbrot {
    pub fn new(width: usize, height: usize, format: Format) -> Self {
        Self {
            width,
            height,
            left: -1.5,
            right: 0.5,
            top: 1.0,
            bottom: -1.0,
            format,
        }
    }
    pub fn width_step(&self) -> f64 {
        (self.right - self.left) / self.width as f64
    }
    pub fn height_step(&self) -> f64 {
        (self.bottom - self.top) / self.height as f64
    }
    pub fn get_format_fn(&self) -> FormatFn {
        output::get_format_fn(self.format)
    }
    pub fn write_header<O: io::Write>(&self, o: &mut O) {
        output::write_header(o, self.width, self.height, self.format)
    }

    pub fn line_buffer(&self, no_lines: usize) -> Vec<u8> {
        match self.format {
            Format::PPM => vec![0_u8; 3 * self.width * no_lines],
            Format::PBM => {
                assert!(self.width % 8 == 0);
                vec![0_u8; self.width / 8 * no_lines]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_simd() {
        fn verify(w: usize, h: usize, is: &[u8], expected: &[u8]) {
            if expected != is {
                for i in 0..h {
                    let b = 3 * w * i;
                    let e = 3 * w * (i + 1);
                    assert_eq!(
                        &is[b..e],
                        &expected[b..e],
                        "line {} differs",
                        i
                    );
                }
            }
        }

        let w = 200;
        let h = 200;
        let mut m = Mandelbrot::new(w, h, Format::PPM);

        let mut v_expected = Vec::new();
        let mut v_simd = Vec::new();
        let mut v_par_simd = Vec::new();
        #[cfg(feature = "ispc")]
        let mut v_ispc = Vec::new();

        {
            scalar::output(&mut v_expected, &mut m, LIMIT);
        }
        {
            simd::output(&mut v_simd, &mut m, LIMIT);
        }
        {
            par_simd::output(&mut v_par_simd, &mut m, LIMIT);
        }
        #[cfg(feature = "ispc")]
        {
            ispc_::output(&mut v_ispc, &mut m, LIMIT);
        }

        assert_eq!(v_expected.len(), 3 * w * h);
        #[cfg(not(target_feature = "fma"))]
        {
            verify(w, h, &v_simd, &v_expected);
            verify(w, h, &v_par_simd, &v_expected);
        }
        if !is_x86_feature_detected!("fma") {
            #[cfg(feature = "ispc")]
            verify(w, h, &v_ispc, &v_expected);
        }
    }
}
