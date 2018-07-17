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
                #[cfg(any(
                    target_arch = "aarch64", target_arch = "arm"))]
                {
                    // FIXME: broken on AArch64
                    // https://bugs.llvm.org/show_bug.cgi?id=36796
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
                #[cfg(not(any(
                    target_arch = "aarch64", target_arch = "arm",
                    all(target_arch = "x86", not(target_feature = "sse2"))
                )))]
                {
                    use llvm::simd_reduce_min;
                    unsafe { simd_reduce_min(self.0) }
                }
                #[cfg(any(
                    target_arch = "aarch64", target_arch = "arm",
                    all(target_arch = "x86", not(target_feature = "sse2"))
                ))]
                {
                    // FIXME: broken on AArch64
                    // https://bugs.llvm.org/show_bug.cgi?id=36796
                    // FIXME: broken on i586-unknown-linux-gnu
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

macro_rules! test_reduction_float_min_max {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        #[cfg(test)]
        interpolate_idents! {
            mod [$id _reduction_min_max_nan] {
                use super::*;
                #[test]
                fn min_element_test() {
                    let n = $elem_ty::NAN;

                    assert_eq!(n.min(-3.), -3.);
                    assert_eq!((-3. as $elem_ty).min(n), -3.);

                    let v0 = $id::splat(-3.);

                    for i in 0..$id::lanes() {
                        let mut v = v0.replace(i, n);
                        if i == $id::lanes() - 1 &&
                            !cfg!(any(
                                target_arch = "arm", target_arch = "aarch64",
                                all(target_arch = "x86", not(target_feature = "sse2"))
                            ))
                        {
                            // FIXME (https://github.com/rust-lang-nursery/stdsimd/issues/408):
                            //
                            // If there is a NaN, the result should always the smallest element,
                            // but currently when the last element is NaN the current
                            // implementation incorrectly returns NaN.
                            //
                            // The targets mentioned above use different codegen that
                            // produces the correct result.
                            //
                            // These asserts detect if this behavior changes
                            assert!(v.min_element().is_nan(), // FIXME: should be -3.
                                       "[C]: nan at {} => {} | {:?}",
                                       i, v.min_element(), v);
                            for j in 0..i {
                                v = v.replace(j, n);
                                assert!(v.min_element().is_nan(), // FIXME: should be -3.
                                        "[D]: nan at {} => {} | {:?}",
                                        i, v.min_element(), v);
                            }
                            break
                        }
                        if i == $id::lanes() - 1 && cfg!(any(
                            target_arch = "arm", target_arch = "aarch64",
                            all(target_arch = "x86", not(target_feature = "sse2"))
                        )) {
                            assert!(false, "this cannot happen");
                        }

                        assert_eq!(v.min_element(), -3.,
                                   "[A]: nan at {} => {} | {:?}",
                                   i, v.min_element(), v);
                        for j in 0..i {
                            v = v.replace(j, n);
                            assert_eq!(v.min_element(), -3.,
                                       "[B]: nan at {} => {} | {:?}",
                                       i, v.min_element(), v);
                        }
                    }
                    // If the vector contains all NaNs the result is NaN:
                    let vn = $id::splat(n);
                    assert!(vn.min_element().is_nan(),
                            "all nans | v={:?} | min={} | is_nan: {}",
                            vn, vn.min_element(), vn.min_element().is_nan());
                }
                #[test]
                fn max_element_test() {
                    let n = $elem_ty::NAN;

                    assert_eq!(n.max(-3.), -3.);
                    assert_eq!((-3. as $elem_ty).max(n), -3.);

                    let v0 = $id::splat(-3.);

                    for i in 0..$id::lanes() {
                        let mut v = v0.replace(i, n);
                        if i == $id::lanes() - 1 &&
                            !cfg!(any(target_arch = "arm", target_arch = "aarch64")
                            ))
                        {
                            // FIXME (https://github.com/rust-lang-nursery/stdsimd/issues/408):
                            //
                            // If there is a NaN, the result should always the largest element,
                            // but currently when the last element is NaN the current
                            // implementation incorrectly returns NaN.
                            //
                            // The targets mentioned above use different codegen that
                            // produces the correct result.
                            //
                            // These asserts detect if this behavior changes
                            assert!(v.max_element().is_nan(), // FIXME: should be -3.
                                       "[C]: nan at {} => {} | {:?}",
                                       i, v.max_element(), v);
                            for j in 0..i {
                                v = v.replace(j, n);
                                assert!(v.max_element().is_nan(), // FIXME: should be -3.
                                           "[D]: nan at {} => {} | {:?}",
                                           i, v.max_element(), v);
                            }
                            break
                        }

                        if i == $id::lanes() - 1 &&
                          cfg!(any(target_arch = "arm", target_arch = "aarch64")) {
                            assert!(false, "this cannot happen");
                        }
                        assert_eq!(v.max_element(), -3.,
                                   "[A]: nan at {} => {} | {:?}",
                                   i, v.max_element(), v);
                        for j in 0..i {
                            v = v.replace(j, n);
                            assert_eq!(v.max_element(), -3.,
                                       "[B]: nan at {} => {} | {:?}",
                                       i, v.max_element(), v);
                        }
                    }
                    // If the vector contains all NaNs the result is NaN:
                    let vn = $id::splat(n);
                    assert!(vn.max_element().is_nan(),
                            "all nans | v={:?} | max={} | is_nan: {}",
                            vn, vn.max_element(), vn.max_element().is_nan());
                }
            }
        }
    }
}
