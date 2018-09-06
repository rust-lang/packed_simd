//! Vectorized mandelbrot
#![allow(non_camel_case_types)]

use packed_simd::*;
use *;

pub type u64s = u64x4;
pub type u32s = u32x4;
pub type f64s = f64x4;

pub fn mandelbrot(c_x: f64s, c_y: f64s, iterations: u32) -> u32s {
    let mut x = c_x;
    let mut y = c_y;

    let mut count = u64s::splat(0);

    for i in 0..iterations {
        let xy = x * y;
        let new_x = x.mul_adde(x, y.mul_adde(-y, c_x));
        let new_y = xy.mul_adde(f64s::splat(2.0), c_y);

        let sum = x.mul_adde(x, y * y);

        // Keep track of those lanes which haven't diverged yet. The other ones
        // will be masked off.
        let undiverged = sum.le(f64s::splat(4.));

        // Stop the iteration if they all diverged. Note that we don't do this
        // check every iteration, since a branch misprediction can hurt more than
        // doing some extra calculations.
        if i % 5 == 0 && undiverged.none() {
            break;
        }

        count += undiverged.select(u64s::splat(1), u64s::splat(0));

        x = new_x;
        y = new_y;
    }
    count.cast()
}

pub fn output<O: io::Write>(o: &mut O, m: &mut Mandelbrot, limit: u32) {
    assert_eq!(
        m.width % f64s::lanes(),
        0,
        "image width = {} is not divisible by the number of vector lanes = {}",
        m.width,
        f64s::lanes()
    );

    let height_step = m.height_step() as f64;
    let height = m.height;
    let block_size = u32s::lanes();
    let width_step = m.width_step() as f64;
    let width = m.width;
    let width_in_blocks = width / block_size;
    let out_fn = m.get_format_fn();

    let adjust = {
        let mut adjust = f64s::splat(0.);
        for i in 0..f64s::lanes() {
            adjust = adjust.replace(i, i as f64);
        }
        adjust
    };

    let mut out = vec![u32s::splat(0); height * width_in_blocks];

    let dur = time::Duration::span(|| {
        for i in 0..height {
            let y = f64s::splat(m.top as f64 + height_step * i as f64);
            for j in (0..width).step_by(block_size) {
                let offset = f64s::splat(j as f64) + adjust;
                let x = f64s::splat(m.left as f64) + width_step * offset;
                let val = simd::mandelbrot(x, y, limit);
                let index = i * width_in_blocks + j / block_size;

                out[index] = val;
            }
        }
    });
    eprintln!("simd: {} ms", dur.num_milliseconds());

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
