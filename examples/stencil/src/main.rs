extern crate stencil_lib;
use stencil_lib::*;

extern crate time;

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
    run("scalar", scalar::scalar);
    run("simd", simd::x8);
    run("simd+par", simd_par::x8_par);

    #[cfg(feature = "ispc")]
    {
        run("ispc", ispc_loops::serial);
        run("ispc+tasks", ispc_loops::tasks);
    }
}
