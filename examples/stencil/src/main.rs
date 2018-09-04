extern crate stencil_lib;
use stencil_lib::*;

extern crate time;

use std::env;

#[rustfmt::skip]
fn run<F>(name: &str, f: F)
where
    F: Fn(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32,
        &[f32; 4], &[f32], &mut [f32], &mut [f32]) -> (),
{
    let mut d = Data::default();
    let t = time::Duration::span(move || d.exec(f));
    println!("{}: {} ms", name, t.num_milliseconds());
}

fn main() {
    let mut args = env::args();
    args.next();
    let alg: usize = args.next().unwrap().parse().unwrap();

    match alg {
        0 => run("scalar", scalar::scalar),
        1 => run("vector", simd::x8),
        2 => run("vector_par", simd_par::x8_par),
        3 => {
            #[cfg(feature = "ispc")]
            {
                run("ispc", ispc_loops::serial);
            }
            #[cfg(not(feature = "ispc"))]
            {
                panic!("error: algorithm requires binary to be compiled with the ispc feature")
            }
        }
        4 => {
            #[cfg(feature = "ispc")]
            {
                run("ispc+tasks", ispc_loops::tasks);
            }
            #[cfg(not(feature = "ispc"))]
            {
                panic!("error: algorithm requires binary to be compiled with the ispc feature")
            }
        }
        _ => panic!("unknown algorithm"),
    }
}
