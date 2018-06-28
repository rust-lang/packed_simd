//! Sealed traits

/// Trait implemented by arrays that can be SIMD types.
#[doc(hidden)]
pub trait SimdArray {
    /// The type of the #[repr(simd)] type.
    type Tuple: Copy + Clone;
    /// The element type of the vector.
    type T;
    /// The number of elements in the array.
    const N: usize;
    /// The type: `[u32; Self::N]`.
    type NT;
}
