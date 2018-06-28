//! Implements `PartialEq` for vector types.

macro_rules! impl_cmp_partial_eq {
    (
        [$elem_ty:ident; $elem_count:expr]:
        $id:ident |
        ($true:expr, $false:expr)
    ) => {
        impl ::cmp::PartialEq<$id> for $id {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                $id::eq(*self, *other).all()
            }
            #[inline]
            fn ne(&self, other: &Self) -> bool {
                $id::ne(*self, *other).all()
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _cmp_PartialEq] {
                use super::*;
                #[test]
                fn partial_eq() {
                    let a = $id::splat($false);
                    let b = $id::splat($true);

                    assert!(a != b);
                    assert!(!(a == b));
                    assert!(a == a);
                    assert!(!(a != a));
                }
            }
        }
    };
}
