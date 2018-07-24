//! The N-body benchmark from the [benchmarks game][bg].
//!
//! [bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/nbody.html#nbody

extern crate nbody_lib;

fn run<O: ::std::io::Write>(o: &mut O, n: usize) {
    let (energy_before, energy_after) = nbody_lib::run(n);

    write!(o, "{:.9}\n", energy_before);
    write!(o, "{:.9}\n", energy_after);
}

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .expect("need one arg")
        .parse()
        .expect("argument should be a usize");
    run(&mut ::std::io::stdout(), n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static OUTPUT: &'static [u8] = include_bytes!("nbody-output.txt");
    #[test]
    fn verify_output() {
        let mut out: Vec<u8> = Vec::new();

        run(&mut out, 1000);

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
