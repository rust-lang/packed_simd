//! Scalar mandelbrot implementation

use ::*;

pub fn mandelbrot(c_x: f32, c_y: f32, max_iter: u32) -> u32 {
    let mut x = c_x;
    let mut y = c_y;
    let mut count = 0;
    while count < max_iter {
        let xy = x * y;
        let xx = x * x;
        let yy = y * y;
        let sum = xx + yy;
        if sum > 4.0 {
            break
        }
        count += 1;
        x = xx - yy + c_x;
        y = xy * 2.0 + c_y;
    }
    count
}

pub fn output<O>(m: &mut Mandelbrot, o: &mut O)
    where O: ::std::io::Write
{
    let height_step = m.height_step();
    let width_step = m.width_step();
    for i in 0..m.height {
        let y = m.top + height_step * i as f32;
        for j in 0..m.width {
            let x = m.left + width_step * j as f32;
            let b = 3*j;
            let e = 3*(j+1);
            let val = scalar::mandelbrot(x, y, LIMIT);
            output_one(&mut m.line[b..e], val);
        }
        o.write(&m.line).unwrap();
    }
}
