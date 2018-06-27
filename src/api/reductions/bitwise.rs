//! Implements portable horizontal bitwise vector reductions.
#![allow(unused)]

macro_rules! impl_reduction_bitwise {
    (
        [$elem_ty:ident; $elem_count:expr]:
        $id:ident |
        ($true:expr, $false:expr)
    ) => {
        impl $id {
            /// Lane-wise bitwise `and` of the vector elements.
            ///
            /// Note: if the vector has one lane, the first element of the
            /// vector is returned.
            #[inline]
            pub fn and(self) -> $elem_ty {
                #[cfg(not(target_arch = "aarch64"))]
                {
                    use llvm::simd_reduce_and;
                    unsafe { simd_reduce_and(self.0) }
                }
                #[cfg(target_arch = "aarch64")]
                {
                    // FIXME: broken on aarch64
                    // https://bugs.llvm.org/show_bug.cgi?id=36796
                    let mut x = self.extract(0) as $elem_ty;
                    for i in 1..$id::lanes() {
                        x &= self.extract(i) as $elem_ty;
                    }
                    x
                }
            }

            /// Lane-wise bitwise `or` of the vector elements.
            ///
            /// Note: if the vector has one lane, the first element of the
            /// vector is returned.
            #[inline]
            pub fn or(self) -> $elem_ty {
                #[cfg(not(target_arch = "aarch64"))]
                {
                    use llvm::simd_reduce_or;
                    unsafe { simd_reduce_or(self.0) }
                }
                #[cfg(target_arch = "aarch64")]
                {
                    // FIXME: broken on aarch64
                    // https://bugs.llvm.org/show_bug.cgi?id=36796
                    let mut x = self.extract(0) as $elem_ty;
                    for i in 1..$id::lanes() {
                        x |= self.extract(i) as $elem_ty;
                    }
                    x
                }
            }

            /// Lane-wise bitwise `xor` of the vector elements.
            ///
            /// Note: if the vector has one lane, the first element of the
            /// vector is returned.
            #[inline]
            pub fn xor(self) -> $elem_ty {
                #[cfg(not(target_arch = "aarch64"))]
                {
                    use llvm::simd_reduce_xor;
                    unsafe { simd_reduce_xor(self.0) }
                }
                #[cfg(target_arch = "aarch64")]
                {
                    // FIXME: broken on aarch64
                    // https://bugs.llvm.org/show_bug.cgi?id=36796
                    let mut x = self.extract(0) as $elem_ty;
                    for i in 1..$id::lanes() {
                        x ^= self.extract(i) as $elem_ty;
                    }
                    x
                }
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _reduction_bitwise] {
                use super::*;

                #[test]
                fn and() {
                    let v = $id::splat($false);
                    assert_eq!(v.and(), $false);
                    let v = $id::splat($true);
                    assert_eq!(v.and(), $true);
                    let v = $id::splat($false);
                    let v = v.replace(0, $true);
                    if $id::lanes() > 1 {
                        assert_eq!(v.and(), $false);
                    } else {
                        assert_eq!(v.and(), $true);
                    }
                    let v = $id::splat($true);
                    let v = v.replace(0, $false);
                    assert_eq!(v.and(), $false);

                }
                #[test]
                fn or() {
                    let v = $id::splat($false);
                    assert_eq!(v.or(), $false);
                    let v = $id::splat($true);
                    assert_eq!(v.or(), $true);
                    let v = $id::splat($false);
                    let v = v.replace(0, $true);
                    assert_eq!(v.or(), $true);
                    let v = $id::splat($true);
                    let v = v.replace(0, $false);
                    if $id::lanes() > 1 {
                        assert_eq!(v.or(), $true);
                    } else {
                        assert_eq!(v.or(), $false);
                    }
                }
                #[test]
                fn xor() {
                    let v = $id::splat($false);
                    assert_eq!(v.xor(), $false);
                    let v = $id::splat($true);
                    if $id::lanes() > 1 {
                        assert_eq!(v.xor(), $false);
                    } else {
                        assert_eq!(v.xor(), $true);
                    }
                    let v = $id::splat($false);
                    let v = v.replace(0, $true);
                    assert_eq!(v.xor(), $true);
                    let v = $id::splat($true);
                    let v = v.replace(0, $false);
                    if $id::lanes() > 1 {
                        assert_eq!(v.xor(), $true);
                    } else {
                        assert_eq!(v.xor(), $false);
                    }
                }
            }
        }
    };
}
