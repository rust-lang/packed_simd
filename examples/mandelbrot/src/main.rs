//! The mandelbrot benchmark from the [benchmarks game][bg].
//!
//! [bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/mandelbrot.html#mandelbrot
#![deny(warnings)]
#![cfg_attr(feature = "cargo-clippy", allow(similar_names))]
extern crate mandelbrot_lib;
use mandelbrot_lib::*;
use std::{env, io, io::Write};

enum Algorithm {
    Scalar,
    Simd,
    ParSimd,
    Ispc,
}

fn run<O: Write>(
    mut o: O, width: usize, height: usize, alg: &Algorithm, format: Format,
) {
    let mut m = Mandelbrot::new(width, height, format);
    m.write_header(&mut o);

    match *alg {
        Algorithm::Scalar => scalar::output(&mut o, &mut m, LIMIT),
        Algorithm::Simd => simd::output(&mut o, &mut m, LIMIT),
        Algorithm::ParSimd => par_simd::output(&mut o, &mut m, LIMIT),
        Algorithm::Ispc => {
            #[cfg(feature = "ispc")]
            {
                ispc_::output(&mut o, &mut m, LIMIT)
            }
            #[cfg(not(feature = "ispc"))]
            {
                panic!("binary wasn't compiled with --feature=ispc");
            }
        }
    }
}

fn main() {
    let mut args = env::args();
    args.next();

    // width height alg fmt
    let width = args.next().unwrap().parse().unwrap();

    let height =
        if let Some(h) = args.next() { h.parse().unwrap() } else { width };

    let alg = if let Some(v) = args.next() {
        match v.parse().unwrap() {
            0 => Algorithm::Scalar,
            1 => Algorithm::Simd,
            2 => Algorithm::ParSimd,
            3 => Algorithm::Ispc,
            v => panic!("unknown algorithm value: {}", v),
        }
    } else {
        Algorithm::Simd
    };

    let fmt = if let Some(f) = args.next() {
        match f.parse().unwrap() {
            0 => output::Format::PBM,
            1 => output::Format::PPM,
            v => panic!("unknown output format value: {}", v),
        }
    } else {
        output::Format::PBM
    };

    run(io::stdout(), width, height, &alg, fmt);
}

#[cfg(test)]
mod tests {
    use super::*;
    static OUTPUT: &'static [u8] = include_bytes!("mandelbrot-output.txt");
    const WIDTH: usize = 200;
    const HEIGHT: usize = 200;
    #[test]
    fn verify_output_scalar() {
        let mut out: Vec<u8> = Vec::new();

        run(&mut out, WIDTH, HEIGHT, &Algorithm::Scalar, output::Format::PBM);

        assert_eq!(out.len(), OUTPUT.len());
        if out != OUTPUT {
            for i in 0..out.len() {
                assert_eq!(
                    out[i], OUTPUT[i],
                    "byte {} differs - is: {:#08b} - should: {:#08b}",
                    i, out[i], OUTPUT[i]
                );
            }
        }
    }
    #[test]
    fn verify_output_simd() {
        let mut out: Vec<u8> = Vec::new();

        run(&mut out, WIDTH, HEIGHT, &Algorithm::Simd, output::Format::PBM);

        assert_eq!(out.len(), OUTPUT.len());
        if out != OUTPUT {
            for i in 0..out.len() {
                assert_eq!(
                    out[i], OUTPUT[i],
                    "byte {} differs - is: {:#08b} - should: {:#08b}",
                    i, out[i], OUTPUT[i]
                );
            }
        }
    }
    #[test]
    fn verify_output_par_simd() {
        let mut out: Vec<u8> = Vec::new();

        run(&mut out, WIDTH, HEIGHT, &Algorithm::ParSimd, output::Format::PBM);

        assert_eq!(out.len(), OUTPUT.len());
        if out != OUTPUT {
            for i in 0..out.len() {
                assert_eq!(
                    out[i], OUTPUT[i],
                    "byte {} differs - is: {:#08b} - should: {:#08b}",
                    i, out[i], OUTPUT[i]
                );
            }
        }
    }
}
