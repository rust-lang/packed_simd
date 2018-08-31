//! Vectorized mandelbrot

use rayon::prelude::*;
use *;

pub fn output<O: io::Write>(o: &mut O, m: &mut Mandelbrot, limit: u32) {
    use simd::f64s;

    assert_eq!(
        m.width % f64s::lanes(),
        0,
        "image width = {} is not divisible by the number of vector lanes = {}",
        m.width,
        f64s::lanes()
    );

    let height_step = m.height_step() as f64;
    let width_step = m.width_step() as f64;
    let out_fn = m.get_format_fn();

    let mut adjust = f64s::splat(0.);
    for i in 0..f64s::lanes() {
        adjust = adjust.replace(i, i as f64);
    }

    let mut line_buffer = m.line_buffer(m.height);
    let line_len = line_buffer.len() / m.height;

    line_buffer.par_chunks_mut(line_len).enumerate().for_each(
        |(i, mut line_buffer)| {
            let y = f64s::splat(m.top as f64 + height_step * i as f64);
            for j in (0..m.width).step_by(f64s::lanes()) {
                let offset: f64s = f64s::splat(j as f64) + adjust;
                let x = f64s::splat(m.left as f64) + width_step * offset;
                let ret = simd::mandelbrot(x, y, limit);
                for k in 0..f64s::lanes() {
                    out_fn(&mut line_buffer, j + k, ret.extract(k));
                }
            }
        },
    );
    o.write_all(&line_buffer).unwrap();
}
