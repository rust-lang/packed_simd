//! Implements `PartialOrd` for vector types.
//!
//! This implements a lexicographical order.

macro_rules! impl_cmp_partial_ord {
    (
        [$elem_ty:ident; $elem_count:expr]: $id:ident
    ) => {
        impl ::cmp::PartialOrd<$id> for $id {
            #[inline]
            fn cmp(&self, other: &Self) -> Option<::cmp::Ordering> {
                if PartialEq::eq(self, other) {
                    Some(::cmp::Ordering::Equal)
                } else if PartialOrd::lt(self, other) {
                    Some(::cmp::Ordering::Less)
                } else if PartialOrd::gt(self, other) {
                    Some(::cmp::Ordering::Greater)
                } else {
                    None
                }
            }
            #[inline]
            fn lt(&self, other: &Self) -> bool {
                self.lt(other).extract(0)
            }
            #[inline]
            fn le(&self, other: &Self) -> bool {
                self.le(other).all()
            }
            #[inline]
            fn ge(&self, other: &Self) -> bool {
                self.ge(other).all()
            }
            #[inline]
            fn gt(&self, other: &Self) -> bool {
                self.gt(other).extract(0)
            }
        }
    };
}


macro_rules! test_cmp_partial_ord_int {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        #[cfg(test)]
        interpolate_idents! {
            mod [$id _cmp_PartialOrd] {
                use super::*;
                #[test]
                fn partial_ord() {
                    // constant values
                    let a = $id::splat(0);
                    let b = $id::splat(1);

                    ::test_utils::test_lt(a, b);
                    ::test_utils::test_cmp(a, b, Some(::cmp::Ordering::Less));
                    ::test_utils::test_cmp(b, a, Some(::cmp::Ordering::Greater));
                    ::test_utils::test_le(a, b);
                    ::test_utils::test_le(a, a);
                    ::test_utils::test_le(b, b);
                    ::test_utils::test_cmp(a, a, Some(::cmp::Ordering::Equal));
                    ::test_utils::test_cmp(b, b, Some(::cmp::Ordering::Equal));

                    // variable values: a = [0, 1, 2, 3]; b = [3, 2, 1, 0]
                    let mut a = $id::splat(0);
                    let mut b = $id::splat(0);
                    for i in 0..$id::lanes() {
                        a = a.replace(i, i as $elem_ty);
                        b = a.replace(i, ($id::lanes() - i) as $elem_ty);
                    }
                    test_lt(a, b);
                    ::test_utils::test_cmp(a, b, Some(::cmp::Ordering::Less));
                    ::test_utils::test_cmp(b, a, Some(::cmp::Ordering::Greater));
                    test_le(a, b);
                    test_le(a, a);
                    test_le(b, b);
                    ::test_utils::test_cmp(a, a, Some(::cmp::Ordering::Equal));
                    ::test_utils::test_cmp(b, b, Some(::cmp::Ordering::Equal));

                    // variable values: a = [0, 1, 2, 3]; b = [0, 1, 2, 4]
                    let mut b = a;
                    b = b.replace($id::lanes()-1, a.extract($id::lanes() - 1) + 1 as $elem_ty);
                    test_lt(a, b);
                    ::test_utils::test_cmp(a, b, Some(::cmp::Ordering::Less));
                    ::test_utils::test_cmp(b, a, Some(::cmp::Ordering::Greater));
                    test_le(a, b);
                    test_le(b, b);
                    ::test_utils::test_cmp(a, a, Some(::cmp::Ordering::Equal));
                    ::test_utils::test_cmp(b, b, Some(::cmp::Ordering::Equal));

                    if $id::lanes() > 1 {
                        // variable values a = [0, 1, 0, 0]; b = [0, 1, 2, 3]
                        let b = a;
                        let mut a = $id::splat(0);
                        a = a.replace(1, 1 as $elem_ty);
                        test_lt(a, b);
                        test_le(a, b);
                        test_le(a, a);

                        ::test_utils::test_cmp(a, a, Some(::cmp::Ordering::Equal));
                        ::test_utils::test_cmp(b, b, Some(::cmp::Ordering::Equal));
                    }

                    if $id::lanes() > 2 {
                        // variable values: a = [0, 1, 2, 3]; b = [0, 1, 3, 2]
                        let mut b = a;
                        b = b.replace(2, a.extract($id::lanes() - 1) + 1 as $elem_ty);
                        test_lt(a, b);
                        test_le(a, b);
                        test_le(b, b);

                        ::test_utils::test_cmp(a, a, Some(::cmp::Ordering::Equal));
                        ::test_utils::test_cmp(b, b, Some(::cmp::Ordering::Equal));
                    }
                }
            }
        }
    }
}
