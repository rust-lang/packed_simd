//! Implements portable horizontal float vector arithmetic reductions.

macro_rules! impl_reduction_float_arithmetic {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl $id {
            /// Horizontal sum of the vector elements.
            ///
            /// The intrinsic performs a tree-reduction of the vector elements.
            /// That is, for an 8 element vector:
            ///
            /// > ((x0 + x1) + (x2 + x3)) + ((x4 + x5) + (x6 + x7))
            ///
            /// If one of the vector element is `NaN` the reduction returns
            /// `NaN`. The resulting `NaN` is not required to be equal to any
            /// of the `NaN`s in the vector.
            #[inline]
            pub fn sum(self) -> $elem_ty {
                #[cfg(not(target_arch = "aarch64"))] {
                    use llvm::simd_reduce_add_ordered;
                    unsafe { simd_reduce_add_ordered(self.0, 0 as $elem_ty) }
                }
                #[cfg(target_arch = "aarch64")] {
                    // FIXME: broken on AArch64
                    // https://bugs.llvm.org/show_bug.cgi?id=36796
                    let mut x = self.extract(0) as $elem_ty;
                    for i in 1..$id::lanes() {
                        x += self.extract(i) as $elem_ty;
                    }
                    x
                }
            }

            /// Horizontal product of the vector elements.
            ///
            /// The intrinsic performs a tree-reduction of the vector elements.
            /// That is, for an 8 element vector:
            ///
            /// > ((x0 * x1) * (x2 * x3)) * ((x4 * x5) * (x6 * x7))
            ///
            /// If one of the vector element is `NaN` the reduction returns
            /// `NaN`. The resulting `NaN` is not required to be equal to any
            /// of the `NaN`s in the vector.
            #[inline]
            pub fn product(self) -> $elem_ty {
                #[cfg(not(target_arch = "aarch64"))] {
                    use llvm::simd_reduce_mul_ordered;
                    unsafe { simd_reduce_mul_ordered(self.0, 1 as $elem_ty) }
                }
                #[cfg(target_arch = "aarch64")] {
                    // FIXME: broken on AArch64
                    // https://bugs.llvm.org/show_bug.cgi?id=36796
                    let mut x = self.extract(0) as $elem_ty;
                    for i in 1..$id::lanes() {
                        x *= self.extract(i) as $elem_ty;
                    }
                    x
                }
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _reduction_float_arith] {
                use super::*;
                fn alternating(x: usize) -> $id {
                    let mut v = $id::splat(1 as $elem_ty);
                    for i in 0..$id::lanes() {
                        if i % x == 0 {
                            v = v.replace(i, 2 as $elem_ty);
                        }
                    }
                    v
                }

                #[test]
                fn sum() {
                    let v = $id::splat(0 as $elem_ty);
                    assert_eq!(v.sum(), 0 as $elem_ty);
                    let v = $id::splat(1 as $elem_ty);
                    assert_eq!(v.sum(), $id::lanes() as $elem_ty);
                    let v = alternating(2);
                    assert_eq!(v.sum(), ($id::lanes() / 2 + $id::lanes()) as $elem_ty);
                }
                #[test]
                fn product() {
                    let v = $id::splat(0 as $elem_ty);
                    assert_eq!(v.product(), 0 as $elem_ty);
                    let v = $id::splat(1 as $elem_ty);
                    assert_eq!(v.product(), 1 as $elem_ty);
                    let f = match $id::lanes() {
                        64 => 16,
                        32 => 8,
                        16 => 4,
                        _ => 2,
                    };
                    let v = alternating(f);
                    assert_eq!(
                        v.product(),
                        (2_usize.pow(($id::lanes() / f) as u32) as $elem_ty)
                    );
                }
            }
        }
    }
}
