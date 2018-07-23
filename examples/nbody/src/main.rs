//! The N-body benchmark from the [benchmarks game][bg].
//!
//! [bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/nbody.html#nbody.
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
