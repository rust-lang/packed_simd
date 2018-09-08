//! Vectorized mandelbrot

use rayon::prelude::*;
use simd::u32s;
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

    let block_size = u32s::lanes();
    let height = m.height;
    let width = m.width;
    let width_in_blocks = width / block_size;
    let height_step = m.height_step() as f64;
    let width_step = m.width_step() as f64;
    let out_fn = m.get_format_fn();

    let mut adjust = f64s::splat(0.);
    for i in 0..f64s::lanes() {
        adjust = adjust.replace(i, i as f64);
    }

    let mut out = vec![u32s::splat(0); height * width_in_blocks];

    let dur = time::Duration::span(|| {
        out.par_chunks_mut(width_in_blocks).enumerate().for_each(
            |(i, line)| {
                let y = f64s::splat(m.top as f64 + height_step * i as f64);
                for j in (0..m.width).step_by(f64s::lanes()) {
                    let offset: f64s = f64s::splat(j as f64) + adjust;
                    let x = f64s::splat(m.left as f64) + width_step * offset;
                    let val = simd::mandelbrot(x, y, limit);
                    line[j / block_size] = val;
                }
            },
        )
    });
    eprintln!("par_simd: {} ms", dur.num_milliseconds());

    let mut line_buffer = m.line_buffer(1);
    for i in 0..height {
        for j in (0..width).step_by(block_size) {
            let val = &out[i * width_in_blocks + j / block_size];
            for k in 0..block_size {
                out_fn(&mut line_buffer, j + k, val.extract(k));
            }
        }
        o.write_all(&line_buffer).unwrap();
    }
}
