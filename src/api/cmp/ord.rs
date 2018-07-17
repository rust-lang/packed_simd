//! Implements `Ord` for vector types.

macro_rules! impl_cmp_ord {
    (
        [$elem_ty:ident; $elem_count:expr]:
        $id:ident |
        ($true:expr, $false:expr)
    ) => {
        impl ::cmp::Ord for $id {
            #[inline]
            fn cmp(&self, other: &Self) -> ::cmp::Ordering {
                match self.partial_cmp(other) {
                    Some(x) => x,
                    None => { unsafe { ::hint::unreachable_unchecked() } },
                }
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _cmp_ord] {
                use super::*;
                #[test]
                fn eq() {
                    fn foo<E: ::cmp::Ord>(_: E) {}
                    let a = $id::splat($false);
                    foo(a);
                }
            }
        }
    };
}
