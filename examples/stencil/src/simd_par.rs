//! SIMD+Rayon implementation.
use rayon::prelude::*;
use simd::step_x8;

pub fn x8_par(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    assert!((z1 - z0) <= n_z);
    for t in t0..t1 {
        if t & 1 == 0 {
            a_odd
                .par_chunks_mut((n_x * n_y) as usize)
                .enumerate()
                .skip(z0 as usize)
                .take((z1 - z0) as usize)
                .for_each(|(z, a_odd)| {
                    let z = z as i32;
                    #[rustfmt::skip]
                    step_x8(
                        x0, x1, y0, y1, z, z + 1, n_x, n_y, n_z,
                        coef, vsq, a_even, a_odd,
                    );
                });
        } else {
            a_even
                .par_chunks_mut((n_x * n_y) as usize)
                .enumerate()
                .skip(z0 as usize)
                .take((z1 - z0) as usize)
                .for_each(|(z, a_even)| {
                    let z = z as i32;
                    #[rustfmt::skip]
                    step_x8(
                        x0, x1, y0, y1, z, z + 1, n_x, n_y, n_z,
                        coef, vsq, a_odd, a_even,
                    );
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::x8_par;
    use scalar::scalar;
    use {assert_data_eq, Data};

    #[test]
    fn simd_par_verify() {
        let mut data_simd_par = Data::default();
        data_simd_par.exec(x8_par);

        let mut data_scalar = Data::default();
        data_scalar.exec(scalar);

        assert_data_eq(&data_simd_par, &data_scalar);
    }
}
