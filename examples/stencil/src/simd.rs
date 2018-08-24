//! SIMD implementation

use packed_simd::*;

pub fn step_x8(
    x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32, n_x: i32, n_y: i32,
    _n_z: i32, coef: &[f32; 4], vsq: &[f32], a_in: &[f32], a_out: &mut [f32],
) {
    assert!((x1 - x0) % f32x8::lanes() as i32 == 0);
    let n_xy = n_x * n_y;
    for z in z0..z1 {
        for y in y0..y1 {
            for x in (x0..x1).step_by(f32x8::lanes()) {
                let out_idx = x + y * n_x;
                let index: i32 = z * n_xy + out_idx;
                macro_rules! a_cur {
                    ($x:expr, $y:expr, $z:expr) => {
                        f32x8::from_slice_unaligned(
                            &a_in[(index + $x + $y * n_x + $z * n_xy)
                                      as usize..],
                        )
                    };
                }
                let cur_0 = a_cur!(0, 0, 0);
                let mut div: f32x8 = coef[0] * cur_0;
                for i in 1..4 {
                    let i: i32 = i;
                    div += coef[i as usize]
                        * (a_cur!(i, 0, 0)
                            + a_cur!(-i, 0, 0)
                            + a_cur!(0, i, 0)
                            + a_cur!(0, -i, 0)
                            + a_cur!(0, 0, i)
                            + a_cur!(0, 0, -i));
                }

                let r =
                    2. * cur_0
                        - f32x8::from_slice_unaligned(
                            &a_out[out_idx as usize..],
                        )
                        + f32x8::from_slice_unaligned(&vsq[index as usize..])
                            * div;
                r.write_to_slice_unaligned(&mut a_out[out_idx as usize..]);
            }
        }
    }
}

pub fn x8(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    for t in t0..t1 {
        if t & 1 == 0 {
            a_odd
                .chunks_mut((n_x * n_y) as usize)
                .enumerate()
                .skip(z0 as usize)
                .take((z1 - z0) as usize)
                .for_each(|(z, a_odd)| {
                    let z = z as i32;
                    step_x8(
                        x0,
                        x1,
                        y0,
                        y1,
                        z,
                        z + 1,
                        n_x,
                        n_y,
                        n_z,
                        coef,
                        vsq,
                        a_even,
                        a_odd,
                    );
                });
        } else {
            a_even
                .chunks_mut((n_x * n_y) as usize)
                .enumerate()
                .skip(z0 as usize)
                .take((z1 - z0) as usize)
                .for_each(|(z, a_even)| {
                    let z = z as i32;
                    step_x8(
                        x0,
                        x1,
                        y0,
                        y1,
                        z,
                        z + 1,
                        n_x,
                        n_y,
                        n_z,
                        coef,
                        vsq,
                        a_odd,
                        a_even,
                    );
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::x8;
    use scalar::scalar;
    use {assert_data_eq, Data};

    #[test]
    fn simd_scalar_verify() {
        let mut data_simd = Data::default();
        data_simd.exec(x8);

        let mut data_scalar = Data::default();
        data_scalar.exec(scalar);

        assert_data_eq(&data_simd, &data_scalar);
    }

    #[cfg(feature = "ispc")]
    #[test]
    fn simd_ispc_verify() {
        use ispc_loops::serial;

        let mut data_simd = Data::default();
        data_simd.exec(x8);

        let mut data_ispc = Data::default();
        data_ispc.exec(serial);

        assert_data_eq(&data_simd, &data_ispc);
    }
}
