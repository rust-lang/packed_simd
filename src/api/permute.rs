//! Implements portable vector permutes with immediate indices.

/// Permute vector elements.
///
/// This macro returns a new vector that contains a permutation of the elements
/// in the vector: `permute!(vec, [indices...])`.
///
/// The number of `indices` must be a power-of-two in range `[0, 64)`, since
/// currently, the largest vector supported by the library has 64 lanes. The
/// length of the resulting vector equals the number of indices provided.
///
/// The indices must be in range `[0, N)` where `N` is the number of lanes of
/// `vec`. in the input vectors. The indices `i` in range `[0, N)` refer to the
/// `i`-th element of `vec`.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate packed_simd;
/// # use packed_simd::*;
/// # fn main() {
/// // Permute allows reordering the elements of a vector:
/// let x = i32x4::new(1, 2, 3, 4);
/// let r = permute!(x, [2, 1, 3, 0]);
/// assert_eq!(r, i32x4::new(3, 2, 4, 1));
///
/// // The resulting vector can be smaller than the input:
/// let r = permute!(x, [1, 3]);
/// assert_eq!(r, i32x2::new(2, 4));
///
/// // Equal:
/// let r = permute!(x, [1, 3, 2, 0]);
/// assert_eq!(r, i32x4::new(2, 4, 3, 1));
///
/// // Or larger:
/// let r = permute!(x, [1, 3, 2, 2, 1, 3, 2, 2]);
/// assert_eq!(r, i32x8::new(2, 4, 3, 3, 2, 4, 3, 3));
/// // At most 2 * the number of lanes in the input vector.
/// # }
/// ```
#[macro_export]
macro_rules! permute {
    ($vec:expr, [$($l:expr),*]) => {
        match $vec {
            v => shuffle!(v, v, [$($l),*])
        }
    };
}
