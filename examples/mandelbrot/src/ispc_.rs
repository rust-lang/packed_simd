//! Includes the ISPC implementations.
use *;

ispc_module!(mandelbrot);

pub fn output<O: io::Write>(o: &mut O, m: &mut Mandelbrot, limit: u32) {
    let out_fn = m.get_format_fn();

    let mut out = Vec::<i32>::with_capacity(m.height * m.width);
    unsafe {
        mandelbrot::mandelbrot_ispc(
            m.left,
            m.bottom,
            m.right,
            m.top,
            m.height as i32,
            m.width as i32,
            limit as i32,
            out.as_mut_ptr() as *mut i32,
        );
        out.set_len(m.height * m.width);
    }

    let mut line_buffer = m.line_buffer(1);
    for i in 0..m.height {
        for j in 0..m.width {
            out_fn(&mut line_buffer, j, out[j + i * m.width] as u32);
        }
        o.write_all(&line_buffer).unwrap();
    }
}
