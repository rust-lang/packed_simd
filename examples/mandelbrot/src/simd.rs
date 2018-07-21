//! Vectorized mandelbrot

use ::packed_simd::*;
use ::*;

pub fn mandelbrot(c_x: f32x4, c_y: f32x4, max_iter: u32) -> u32x4 {
    let mut x = c_x;
    let mut y = c_y;

    let mut count = u32x4::splat(0);
    let max_iter = u32x4::splat(max_iter);

    loop {
        let mask = count.ge(max_iter);
        if mask.all() { break; }

        let xx = x * x;
        let yy = y * y;
        let sum = xx + yy;

        let mask = !sum.gt(f32x4::splat(4.)) & !mask;
        if mask.none() {
            break;
        }

        count += mask.select(u32x4::splat(1), u32x4::splat(0));

        let xy = x * y;
        x = mask.select(xx - yy + c_x, x);
        y = mask.select(xy * 2.0 + c_y, y);
    }
    count
}

pub fn output<O>(m: &mut Mandelbrot, o: &mut O)
    where O: ::std::io::Write
{
    #[allow(non_camel_case_types)]
    type f32s = f32x4;

    assert_eq!(m.width % f32s::lanes(), 0,
               "image width = {} is not divisible by the number of vector lanes = {}",
               m.width, f32s::lanes());

    let height_step = m.height_step();
    let width_step = m.width_step();
    let mut adjust = f32s::splat(0.);
    for i in 0..f32s::lanes() {
        adjust = adjust.replace(i, i as f32);
    }

    for i in 0..m.height {
        let y = f32s::splat(m.top + height_step * i as f32);
        for j in (0..m.width).step_by(f32s::lanes()) {
            let offset: f32s = f32s::splat(j as f32) + adjust;
            let x = f32s::splat(m.left) + width_step * offset;
            let ret = simd::mandelbrot(x, y, LIMIT);
            for k in 0..f32s::lanes() {
                let b = 3 * (j + k);
                let e = 3 * (j + k + 1);
                output_one(&mut m.line[b..e], ret.extract(k));
            }
        }
        o.write(&m.line).unwrap();
    }
}

