#![feature(stmt_expr_attributes)]
#![deny(warnings)]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        similar_names,
        cast_precision_loss,
        cast_sign_loss,
        too_many_arguments,
        cast_possible_wrap,
        cast_possible_truncation
    )
)]

extern crate packed_simd;
extern crate rayon;

#[cfg(feature = "ispc")]
#[macro_use]
extern crate ispc;

#[cfg(feature = "ispc")]
pub mod ispc_loops;
pub mod scalar;
pub mod simd;
pub mod simd_par;

#[derive(Clone, PartialEq, Debug)]
pub struct Data {
    a: (Vec<f32>, Vec<f32>),
    vsq: Vec<f32>,
    coeff: [f32; 4],
    n: (i32, i32, i32),
    t: (i32, i32),
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Data {
    pub fn default() -> Self {
        // ISPC uses this but it takes too long on travis
        // Self::from_bounds(6, 4, 128, 128, 128)
        Self::from_bounds(6, 4, 128, 128, 128)
    }

    pub fn from_bounds(
        max_t: i32, width: i32, n_x: i32, n_y: i32, n_z: i32,
    ) -> Self {
        #[rustfmt::skip]
        Self::new(
            0, max_t,
            width, n_x - width, width, n_y - width, width, n_z - width,
            n_x, n_y, n_z,
        )
    }

    /// Initializes data
    pub fn new(
        t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32,
        z1: i32, n_x: i32, n_y: i32, n_z: i32,
    ) -> Self {
        let n = (n_x * n_y * n_z) as usize;
        let mut data = Self {
            a: (vec![0_f32; n], vec![0_f32; n]),
            vsq: vec![0_f32; n],
            coeff: [0.5, -0.25, 0.125, -0.0625],
            n: (n_x, n_y, n_z),
            t: (t0, t1),
            x: (x0, x1),
            y: (y0, y1),
            z: (z0, z1),
        };

        data.reinit();
        data
    }

    pub fn reinit(&mut self) {
        let mut offset: usize = 0;
        for z in 0..self.n.2 {
            for y in 0..self.n.1 {
                for x in 0..self.n.0 {
                    self.a.0[offset] = if x < self.n.0 / 2 {
                        x as f32 / self.n.0 as f32
                    } else {
                        y as f32 / self.n.1 as f32
                    };
                    self.a.1[offset] = 0.;
                    self.vsq[offset] = (x * y * z) as f32
                        / (self.n.0 * self.n.1 * self.n.2) as f32;
                    offset += 1;
                }
            }
        }
    }

    #[rustfmt::skip]
    pub fn exec<F>(&mut self, f: F)
    where
        F: Fn(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32,
            &[f32; 4], &[f32], &mut [f32], &mut [f32]) -> (),
    {
        f(
            self.t.0, self.t.1,
            self.x.0, self.x.1,
            self.y.0, self.y.1,
            self.z.0, self.z.1,
            self.n.0, self.n.1, self.n.2,
            &self.coeff, &self.vsq, &mut self.a.0, &mut self.a.1,
        );
    }
}

#[cfg(test)]
fn assert_data_eq(a: &Data, b: &Data) {
    if a == b {
        return;
    }
    assert_eq!(a.coeff, b.coeff, "coeffs differ");
    assert_eq!(a.n, b.n, "n differ");
    assert_eq!(a.t, b.t, "t differ");
    assert_eq!(a.x, b.x, "x differ");
    assert_eq!(a.y, b.y, "y differ");
    assert_eq!(a.z, b.z, "z differ");

    for z in 0..a.n.2 {
        for y in 0..a.n.1 {
            for x in 0..a.n.0 {
                let idx = (x + y * a.n.1 + z * a.n.1 * a.n.0) as usize;

                assert_eq!(
                    a.vsq[idx], b.vsq[idx],
                    "vsq diff at idx = {} ({}, {}, {})",
                    idx, x, y, z
                );

                assert_eq!(
                    a.a.0[idx], b.a.0[idx],
                    "a.0 diff at idx = {} ({}, {}, {})",
                    idx, x, y, z
                );

                assert_eq!(
                    a.a.1[idx], b.a.1[idx],
                    "a.1 diff at idx = {} ({}, {}, {})",
                    idx, x, y, z
                );
            }
        }
    }
}
