//! SIMD implementation

use f32s;

// Cumulative normal distribution function
#[inline(always)]
fn cnd(x: f32s) -> f32s {
    const INV_SQRT_2PI: f32s = f32s::splat(0.398_942_280_40);

    let l = x.abs();
    let k = 1. / (1. + 0.231_641_9 * l);
    let k2 = k * k;
    let k3 = k2 * k;
    let k4 = k2 * k2;
    let k5 = k3 * k2;
    let w: f32s = 0.319_381_53 * k - 0.356_563_782 * k2
        + 1.781_477_937 * k3
        + -1.821_255_978 * k4
        + 1.330_274_429 * k5;
    let w = w * INV_SQRT_2PI * (-l * l * 0.5).exp();

    x.gt(f32s::splat(0.)).select(1. - w, w)
}

pub fn black_scholes(
    sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
    result: &mut [f32], count: usize,
) -> f64 {
    assert_eq!(count % f32s::lanes(), 0);
    for i in (0..count).step_by(f32s::lanes()) {
        unsafe {
            let s = f32s::from_slice_unaligned_unchecked(&sa[i..]);
            let x = f32s::from_slice_unaligned_unchecked(&xa[i..]);
            let t = f32s::from_slice_unaligned_unchecked(&ta[i..]);
            let r = f32s::from_slice_unaligned_unchecked(&ra[i..]);
            let v = f32s::from_slice_unaligned_unchecked(&va[i..]);
            let d1 = ((s / x).ln() + (r + v * v * 0.5) * t) / (v * t.sqrt());
            let d2 = d1 - v * t.sqrt();
            let r = s * cnd(d1) - x * (-r * t).exp() * cnd(d2);
            r.write_to_slice_unaligned_unchecked(&mut result[i..]);
        }
    }
    ::sum::fastest(&result)
}

pub fn binomial_put(
    sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
    result: &mut [f32], count: usize,
) -> f64 {
    use BINOMIAL_NUM;

    for i in (0..count).step_by(f32s::lanes()) {
        unsafe {
            let s = f32s::from_slice_unaligned_unchecked(&sa[i..]);
            let x = f32s::from_slice_unaligned_unchecked(&xa[i..]);
            let t = f32s::from_slice_unaligned_unchecked(&ta[i..]);
            let r = f32s::from_slice_unaligned_unchecked(&ra[i..]);
            let v = f32s::from_slice_unaligned_unchecked(&va[i..]);

            let dt = t / BINOMIAL_NUM as f32;
            let u = (v * dt.sqrt()).exp();
            let d = 1. / u;
            let disc = (r * dt).exp();
            let pu = (disc - d) / (u - d);

            let mut vs = [f32s::splat(0.); BINOMIAL_NUM];
            for (j, v) in vs.iter_mut().enumerate() {
                let e = (2_i32 * (j as i32)).wrapping_sub(BINOMIAL_NUM as i32);
                let upow = u.powf(f32s::splat(e as f32));
                *v = f32s::splat(0.).max(x - s * upow);
            }

            for j in (0..BINOMIAL_NUM).rev() {
                for k in 0..j {
                    vs[k] = ((1. - pu) * vs[k] + pu * vs[k + 1]) / disc;
                }
            }

            vs[0].write_to_slice_unaligned_unchecked(&mut result[i..]);
        }
    }
    ::sum::fastest(&result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn black_scholes_ispc() {
        const NOPTS: usize = 1_000_000;
        let mut simd = ::State::new(NOPTS);
        let mut scalar = ::State::new(NOPTS);

        let simd_sum = simd.exec(black_scholes);
        let scalar_sum = scalar.exec(::scalar::black_scholes);

        assert_eq!(simd, scalar);
        assert_eq!(simd_sum, scalar_sum);
    }

    #[test]
    fn binomial_put_scalar() {
        const NOPTS: usize = 1_000_000;
        let mut simd = ::State::new(NOPTS);
        let mut scalar = ::State::new(NOPTS);

        let simd_sum = simd.exec(binomial_put);
        let scalar_sum = scalar.exec(::scalar::binomial_put);

        assert_eq!(simd, scalar);
        assert_eq!(simd_sum, scalar_sum);
    }
}
