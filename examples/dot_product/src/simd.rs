//! Scalar implementation

use packed_simd::f32x4;

pub fn dot_prod(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    assert!(a.len() % 4 == 0);

    a.chunks_exact(4)
        .map(f32x4::from_slice_unaligned)
        .zip(b.chunks_exact(4).map(f32x4::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f32x4>()
        .sum()
}

#[cfg(test)]
#[test]
fn test() {
    crate::test(dot_prod)
}
