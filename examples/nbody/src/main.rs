//! The N-body benchmark from the [benchmarks game][bg].
//!
//! [bg]: https://benchmarksgame.alioth.debian.org/u64q/nbody-description.
//! html#nbody

extern crate nbody_lib;

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .expect("need one arg")
        .parse()
        .expect("argument should be a usize");

    let (energy_before, energy_after) = nbody_lib::run(n);

    println!("{:.9}", energy_before);
    println!("{:.9}", energy_after);
}

