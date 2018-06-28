//! Implements portable horizontal vector min/max reductions.

macro_rules! impl_reduction_min_max {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl $id {
            /// Largest vector element value.
            #[inline]
            pub fn max_element(self) -> $elem_ty {
                #[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
                {
                    use llvm::simd_reduce_max;
                    unsafe { simd_reduce_max(self.0) }
                }
                #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
                {
                    // FIXME: broken on AArch64
                    // https://bugs.llvm.org/show_bug.cgi?id=36796
                    use cmp::Ord;
                    let mut x = self.extract(0);
                    for i in 1..$id::lanes() {
                        x = x.max(self.extract(i));
                    }
                    x
                }
            }

            /// Smallest vector element value.
            #[inline]
            pub fn min_element(self) -> $elem_ty {
                #[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
                {
                    use llvm::simd_reduce_min;
                    unsafe { simd_reduce_min(self.0) }
                }
                #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
                {
                    // FIXME: broken on AArch64
                    // https://bugs.llvm.org/show_bug.cgi?id=36796
                    use cmp::Ord;
                    let mut x = self.extract(0);
                    for i in 1..$id::lanes() {
                        x = x.min(self.extract(i));
                    }
                    x
                }
            }
        }
        #[cfg(test)]
        interpolate_idents! {
            mod [$id _reduction_min_max] {
                use super::*;
                #[test]
                fn max_element() {
                    let v = $id::splat(0 as $elem_ty);
                    assert_eq!(v.max_element(), 0 as $elem_ty);
                    if $id::lanes() > 1 {
                        let v = v.replace(1, 1 as $elem_ty);
                        assert_eq!(v.max_element(), 1 as $elem_ty);
                    }
                    let v = v.replace(0, 2 as $elem_ty);
                    assert_eq!(v.max_element(), 2 as $elem_ty);
                }

                #[test]
                fn min_element() {
                    let v = $id::splat(0 as $elem_ty);
                    assert_eq!(v.min_element(), 0 as $elem_ty);
                    if $id::lanes() > 1 {
                        let v = v.replace(1, 1 as $elem_ty);
                        assert_eq!(v.min_element(), 0 as $elem_ty);
                    }
                    let v = $id::splat(1 as $elem_ty);
                    let v = v.replace(0, 2 as $elem_ty);
                    if $id::lanes() > 1 {
                        assert_eq!(v.min_element(), 1 as $elem_ty);
                    } else {
                        assert_eq!(v.min_element(), 2 as $elem_ty);
                    }
                    if $id::lanes() > 1 {
                        let v = $id::splat(2 as $elem_ty);
                        let v = v.replace(1, 1 as $elem_ty);
                        assert_eq!(v.min_element(), 1 as $elem_ty);
                    }
                }
            }
        }
    };
}
