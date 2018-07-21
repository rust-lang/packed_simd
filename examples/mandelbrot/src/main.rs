extern crate mandelbrot_lib;
use mandelbrot_lib::*;

fn main() {
    let mut args = std::env::args();
    args.next();
    let width = args.next().unwrap().parse().unwrap();
    let height = args.next().unwrap().parse().unwrap();

    let mut m = Mandelbrot::new(width, height);


    let mut o = ::std::io::stdout();
    if args.next().is_none() {
        simd::output(&mut m, &mut o);
    } else {
        scalar::output(&mut m, &mut o);
    }
}
