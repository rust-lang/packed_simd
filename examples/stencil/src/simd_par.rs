//! SIMD+Rayon implementation.

use super::simd::step_x8;
use rayon::prelude::*;

pub fn x8_par(
    t0: i32,
    t1: i32,
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
    z0: i32,
    z1: i32,
    n_x: i32,
    n_y: i32,
    n_z: i32,
    coef: &[f32; 4],
    vsq: &[f32],
    a_even: &mut [f32],
    a_odd: &mut [f32],
) {
    let a_even = a_even.as_mut_ptr() as isize;
    let a_odd = a_odd.as_mut_ptr() as isize;
    for t in t0..t1 {
        (z0..z1).into_par_iter().for_each(|z| {
            let (a_even, a_odd) = unsafe {
                let n = (n_x * n_y * n_z) as usize;
                let a_even: &mut [f32] =
                    ::std::slice::from_raw_parts_mut(a_even as *mut f32, n);
                let a_odd: &mut [f32] =
                    ::std::slice::from_raw_parts_mut(a_odd as *mut f32, n);
                (a_even, a_odd)
            };
            if t & 1 == 0 {
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
            } else {
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
            }
        });
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
