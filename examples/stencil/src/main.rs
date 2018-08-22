extern crate stencil_lib;
use stencil_lib::*;

extern crate time;

fn main() {
    {
        let mut d = Data::default();
        let t = time::Duration::span(move || d.exec(scalar::scalar));
        println!("scalar: {} ms", t.num_milliseconds());
    }

    {
        let mut d = Data::default();
        let t = time::Duration::span(move || d.exec(simd::x8));
        println!("simd: {} ms", t.num_milliseconds());
    }

    {
        let mut d = Data::default();
        let t = time::Duration::span(move || d.exec(simd_par::x8_par));
        println!("simd+par: {} ms", t.num_milliseconds());
    }

    #[cfg(feature = "ispc")]
    {
        {
            let mut d = Data::default();
            let t = time::Duration::span(move || d.exec(ispc_loops::serial));
            println!("ispc: {} ms", t.num_milliseconds());
        }
        {
            let mut d = Data::default();
            let t = time::Duration::span(move || d.exec(ispc_loops::tasks));
            println!("ispc+task: {} ms", t.num_milliseconds());
        }
    }
}
