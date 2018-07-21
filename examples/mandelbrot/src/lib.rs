//! Mandelbrot

extern crate packed_simd;

pub mod scalar;
pub mod simd;

const COLOURS: &'static [(f32, f32, f32)] = &[
    (0.0, 7.0, 100.0),
    (32.0, 107.0, 203.0),
    (237.0, 255.0, 255.0),
    (255.0, 170.0, 0.0),
    (0.0, 2.0, 0.0)
];
const SCALE: f32 = 12.0;
const LIMIT: u32 = 100;

pub struct Mandelbrot {
    // output image width/height:
    pub width: usize,
    pub height: usize,
    // region:
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    // line buffer
    pub line: Vec<u8>

}

impl Mandelbrot {
    pub fn new(width: usize, height: usize) -> Self {
        println!("P6 {} {} 255", width, height);

        Mandelbrot {
            width,
            height,
            left: -2.2,
            right: 1.2,
            top: 1.0,
            bottom: -1.0,
            line: vec![0; 3 * width]
        }
    }
    pub fn width_step(&self) -> f32 { (self.right - self.left) / self.width as f32 }
    pub fn height_step(&self) -> f32 { (self.bottom - self.top) / self.height as f32 }
}

pub fn output_one(buf: &mut [u8], val: u32) {
    assert_eq!(buf.len(), 3);
    let (r, g, b) = if val == LIMIT {
        (0, 0, 0)
    } else {
        let val = (val as f32 % SCALE) * (COLOURS.len() as f32) / SCALE;
        let left = val as usize % COLOURS.len();
        let right = (left + 1) % COLOURS.len();

        let p = val - left as f32;
        let (r1, g1, b1) = COLOURS[left];
        let (r2, g2, b2) = COLOURS[right];
        (
            (r1 + (r2 - r1) * p) as u8,
            (g1 + (g2 - g1) * p) as u8,
            (b1 + (b2 - b1) * p) as u8,
        )
    };
    buf[0] = r;
    buf[1] = g;
    buf[2] = b;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_simd() {
        let w = 100;
        let h = 100;
        let mut m = Mandelbrot::new(w, h);

        let mut v_expected = Vec::new();
        let mut v_simd = Vec::new();

        {
            simd::output(&mut m, &mut v_simd);
        }
        {
            scalar::output(&mut m, &mut v_expected);
        }
        assert_eq!(v_expected.len(), 3 * w * h);
        if v_expected != v_simd {
            for i in 0..h {
                let b = 3*w*i;
                let e = 3*w*(i+1);
                assert_eq!(&v_simd[b..e], &v_expected[b..e],
                           "line {} differs", i);
            }
        }
    }
}
