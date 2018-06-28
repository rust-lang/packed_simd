//! Sealed traits

/// Trait implemented by arrays that can be SIMD types
pub trait SimdArray {
    type Tuple: Copy + Clone;
    type T;
    const N: usize;
}
