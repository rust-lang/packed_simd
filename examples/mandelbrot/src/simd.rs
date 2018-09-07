//! Vectorized mandelbrot
#![allow(non_camel_case_types)]

use packed_simd::*;
use *;

pub type u64s = u64x4;
pub type u32s = u32x4;
pub type f64s = f64x4;

pub fn mandelbrot(c_x: f64s, c_y: f64s, max_iter: u32) -> u32s {
    let mut x = c_x;
    let mut y = c_y;

    let mut count = u64s::splat(0);
    let max_iter = u64s::splat(u64::from(max_iter));

    loop {
        let mask = count.ge(max_iter);
        if mask.all() {
            break;
        }

        let xx = x * x;
        let yy = y * y;
        let sum = xx + yy;

        let mask = !sum.gt(f64s::splat(4.)) & !mask;
        if mask.none() {
            break;
        }

        count += mask.select(u64s::splat(1), u64s::splat(0));

        let xy = x * y;
        x = mask.select(xx - yy + c_x, x);
        y = mask.select(xy * f64s::splat(2.0) + c_y, y);
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
    let width_step = m.width_step() as f64;
    let out_fn = m.get_format_fn();

    let mut adjust = f64s::splat(0.);
    for i in 0..f64s::lanes() {
        adjust = adjust.replace(i, i as f64);
    }
    let mut line_buffer = m.line_buffer(1);
    for i in 0..m.height {
        let y = f64s::splat(m.top as f64 + height_step * i as f64);
        for j in (0..m.width).step_by(f64s::lanes()) {
            let offset: f64s = f64s::splat(j as f64) + adjust;
            let x = f64s::splat(m.left as f64) + width_step * offset;
            let ret = simd::mandelbrot(x, y, limit);
            for k in 0..f64s::lanes() {
                out_fn(&mut line_buffer, j + k, ret.extract(k));
            }
        }
        o.write_all(&line_buffer).unwrap();
    }
}
