//! Implements `PartialOrd` for vector types.
//!
//! This implements a lexicographical order.

// tests <
#[cfg(all(test, not(feature = "test_none")))]
macro_rules! test_lt {
    ($a:expr, $b: expr) => {
        assert!($a < $b, "{:?}, {:?}", $a, $b);
        assert!($b > $a, "{:?}, {:?}", $a, $b);

        assert!(!($a == $b), "{:?}, {:?}", $a, $b);
        assert!($a != $b, "{:?}, {:?}", $a, $b);

        assert!($a <= $b, "{:?}, {:?}", $a, $b);
        assert!($b >= $a, "{:?}, {:?}", $a, $b);

        // Irreflexivity
        assert!(!($a < $a), "{:?}, {:?}", $a, $b);
        assert!(!($b < $b), "{:?}, {:?}", $a, $b);
        assert!(!($a > $a), "{:?}, {:?}", $a, $b);
        assert!(!($b > $b), "{:?}, {:?}", $a, $b);

        assert!($a <= $a, "{:?}, {:?}", $a, $b);
        assert!($b <= $b, "{:?}, {:?}", $a, $b);
    }
}

// tests <=
#[cfg(all(test, not(feature = "test_none")))]
macro_rules! test_le {
    ($a:expr, $b: expr) => {
        assert!($a <= $b, "{:?}, {:?}", $a, $b);
        assert!($b >= $a, "{:?}, {:?}", $a, $b);

        assert!($a == $b || $a < $b, "{:?}, {:?}", $a, $b);
        assert!($a == $b || $b > $a, "{:?}, {:?}", $a, $b);

        if $a == $b {
            assert!(!($a < $b), "{:?}, {:?}", $a, $b);
            assert!(!($b > $a), "{:?}, {:?}", $a, $b);

            assert!(!($a != $b), "{:?}, {:?}", $a, $b);
        } else {
            assert!($a != $b, "{:?}, {:?}", $a, $b);
            test_lt!($a, $b);
        }
    }
}

// tests partial_cmp
#[cfg(all(test, not(feature = "test_none")))]
macro_rules! test_cmp {
    ($a:expr, $b: expr, $o:expr, $T:ty, $elem_ty:ty, $elem_count:expr) => {
        assert!(
            $elem_count <= 64,
            "array length in these two arrays needs updating"
        );
        let mut arr_a: [$elem_ty; 64] = [Default::default(); 64];
        let mut arr_b: [$elem_ty; 64] = [Default::default(); 64];

        unsafe {
            crate::ptr::write_unaligned(
                arr_a.as_mut_ptr() as *mut PartiallyOrdered<$T>,
                $a,
            )
        }
        unsafe {
            crate::ptr::write_unaligned(
                arr_b.as_mut_ptr() as *mut PartiallyOrdered<$T>,
                $b,
            )
        }
        let expected = arr_a[0..$elem_count].partial_cmp(&arr_b[0..$elem_count]);
        let result = $a.partial_cmp(&$b);
        assert_eq!(expected, result, "{:?}, {:?}", $a, $b);
        assert_eq!($o, result, "{:?}, {:?}", $a, $b);
        match $o {
            Some(::cmp::Ordering::Less) => {
                test_lt!($a, $b);
                test_le!($a, $b);
            }
            Some(::cmp::Ordering::Greater) => {
                test_lt!($b, $a);
                test_le!($b, $a);
            }
            Some(::cmp::Ordering::Equal) => {
                assert!($a == $b, "{:?}, {:?}", $a, $b);
                assert!(!($a != $b), "{:?}, {:?}", $a, $b);
                assert!(!($a < $b), "{:?}, {:?}", $a, $b);
                assert!(!($b < $a), "{:?}, {:?}", $a, $b);
                assert!(!($a > $b), "{:?}, {:?}", $a, $b);
                assert!(!($b > $a), "{:?}, {:?}", $a, $b);

                test_le!($a, $b);
                test_le!($b, $a);
            }
            None => {
                assert!(!($a == $b), "{:?}, {:?}", $a, $b);
                assert!(!($a != $b), "{:?}, {:?}", $a, $b);
                assert!(!($a < $b), "{:?}, {:?}", $a, $b);
                assert!(!($a > $b), "{:?}, {:?}", $a, $b);
                assert!(!($b < $a), "{:?}, {:?}", $a, $b);
                assert!(!($b > $a), "{:?}, {:?}", $a, $b);
                assert!(!($a <= $b), "{:?}, {:?}", $a, $b);
                assert!(!($b <= $a), "{:?}, {:?}", $a, $b);
                assert!(!($a >= $b), "{:?}, {:?}", $a, $b);
                assert!(!($b >= $a), "{:?}, {:?}", $a, $b);
            }
        }
    }
}

macro_rules! impl_cmp_partial_ord {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl_!{
        impl $id {
            /// Returns a wrapper that implements `PartialOrd`.
            #[inline]
            pub fn partial_ord(&self) -> PartiallyOrdered<$id> {
                PartiallyOrdered(*self)
            }
        }

        impl ::cmp::PartialOrd<PartiallyOrdered<$id>>
            for PartiallyOrdered<$id>
        {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<::cmp::Ordering> {
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
                let m_lt = self.0.lt(other.0);
                let m_eq = self.0.eq(other.0);
                for i in 0..$id::lanes() {
                    if m_eq.extract(i) {
                        continue;
                    }
                    return m_lt.extract(i);
                }
                false
            }
            #[inline]
            fn le(&self, other: &Self) -> bool {
                self.lt(other) | PartialEq::eq(self, other)
            }
            #[inline]
            fn ge(&self, other: &Self) -> bool {
                self.gt(other) | PartialEq::eq(self, other)
            }
            #[inline]
            fn gt(&self, other: &Self) -> bool {
                let m_gt = self.0.gt(other.0);
                let m_eq = self.0.eq(other.0);
                for i in 0..$id::lanes() {
                    if m_eq.extract(i) {
                        continue;
                    }
                    return m_gt.extract(i);
                }
                false
            }
        }
        }
    };
}

macro_rules! test_cmp_partial_ord_int {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _cmp_PartialOrd] {
                    use super::*;
                    #[test]
                    fn partial_ord() {
                        // constant values
                        let a = $id::splat(0);
                        let b = $id::splat(1);

                        test_cmp!(a.partial_ord(), b.partial_ord(),
                                 Some(::cmp::Ordering::Less), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Greater), $id, $elem_ty, $elem_count);
                        test_cmp!(a.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), b.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);

                        // variable values: a = [0, 1, 2, 3]; b = [3, 2, 1, 0]
                        let mut a = $id::splat(0);
                        let mut b = $id::splat(0);
                        for i in 0..$id::lanes() {
                            a = a.replace(i, i as $elem_ty);
                            b = b.replace(i, ($id::lanes() - i) as $elem_ty);
                        }
                        test_cmp!(a.partial_ord(), b.partial_ord(),
                                  Some(::cmp::Ordering::Less), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Greater), $id, $elem_ty, $elem_count);
                        test_cmp!(a.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), b.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);

                        // variable values: a = [0, 1, 2, 3]; b = [0, 1, 2, 4]
                        let mut b = a;
                        b = b.replace($id::lanes()-1,
                                      a.extract($id::lanes() - 1) + 1 as $elem_ty);
                        test_cmp!(a.partial_ord(), b.partial_ord(),
                                  Some(::cmp::Ordering::Less), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Greater), $id, $elem_ty, $elem_count);
                        test_cmp!(a.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), b.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);

                        if $id::lanes() > 2 {
                            // variable values a = [0, 1, 0, 0]; b = [0, 1, 2, 3]
                            let b = a;
                            let mut a = $id::splat(0);
                            a = a.replace(1, 1 as $elem_ty);
                            test_cmp!(a.partial_ord(), b.partial_ord(),
                                      Some(::cmp::Ordering::Less), $id, $elem_ty, $elem_count);
                            test_cmp!(b.partial_ord(), a.partial_ord(),
                                      Some(::cmp::Ordering::Greater), $id, $elem_ty, $elem_count);
                            test_cmp!(a.partial_ord(), a.partial_ord(),
                                      Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);
                            test_cmp!(b.partial_ord(), b.partial_ord(),
                                      Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);

                            // variable values: a = [0, 1, 2, 3]; b = [0, 1, 3, 2]
                            let mut b = a;
                            b = b.replace(
                                2, a.extract($id::lanes() - 1) + 1 as $elem_ty
                            );
                            test_cmp!(a.partial_ord(), b.partial_ord(),
                                      Some(::cmp::Ordering::Less), $id, $elem_ty, $elem_count);
                            test_cmp!(b.partial_ord(), a.partial_ord(),
                                      Some(::cmp::Ordering::Greater), $id, $elem_ty, $elem_count);
                            test_cmp!(a.partial_ord(), a.partial_ord(),
                                      Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);
                            test_cmp!(b.partial_ord(), b.partial_ord(),
                                      Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);
                        }
                    }
                }
            }
        }
    };
}

macro_rules! test_cmp_partial_ord_mask {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _cmp_PartialOrd] {
                    use super::*;
                    #[test]
                    fn partial_ord() {
                        // constant values
                        let a = $id::splat(false);
                        let b = $id::splat(true);

                        test_cmp!(a.partial_ord(), b.partial_ord(),
                                  Some(::cmp::Ordering::Less), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Greater), $id, $elem_ty, $elem_count);
                        test_cmp!(a.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), b.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);

                        // variable values:
                        // a = [false, false, false, false];
                        // b = [false, false, false, true]
                        let a = $id::splat(false);
                        let mut b = $id::splat(false);
                        b = b.replace($id::lanes() - 1, true);
                        test_cmp!(a.partial_ord(), b.partial_ord(),
                                  Some(::cmp::Ordering::Less), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Greater), $id, $elem_ty, $elem_count);
                        test_cmp!(a.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), b.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);

                        // variable values:
                        // a = [true, true, true, false];
                        // b = [true, true, true, true]
                        let mut a = $id::splat(true);
                        let b = $id::splat(true);
                        a = a.replace($id::lanes() - 1, false);
                        test_cmp!(a.partial_ord(), b.partial_ord(),
                                  Some(::cmp::Ordering::Less), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Greater), $id, $elem_ty, $elem_count);
                        test_cmp!(a.partial_ord(), a.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);
                        test_cmp!(b.partial_ord(), b.partial_ord(),
                                  Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);

                        if $id::lanes() > 2 {
                            // variable values
                            // a = [false, true, false, false];
                            // b = [false, true, true, true]
                            let mut a = $id::splat(false);
                            let mut b = $id::splat(true);
                            a = a.replace(1, true);
                            b = b.replace(0, false);
                            test_cmp!(a.partial_ord(), b.partial_ord(),
                                      Some(::cmp::Ordering::Less), $id, $elem_ty, $elem_count);
                            test_cmp!(b.partial_ord(), a.partial_ord(),
                                      Some(::cmp::Ordering::Greater), $id, $elem_ty, $elem_count);
                            test_cmp!(a.partial_ord(), a.partial_ord(),
                                      Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);
                            test_cmp!(b.partial_ord(), b.partial_ord(),
                                      Some(::cmp::Ordering::Equal), $id, $elem_ty, $elem_count);
                        }
                    }
                }
            }
        }
    };
}
