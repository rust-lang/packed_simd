//! Test utilities

/// Tests PartialOrd for `a` and `b` where `a < b` is true.
pub fn test_lt<T: PartialOrd>(a: T, b: T) {
    assert!(a < b);
    assert!(b > a);

    assert!(!(a == b));
    assert!(a != b);

    assert!(a <= b);
    assert!(b >= a);

    // Irreflexivity
    assert!(!(a < a));
    assert!(!(b < b));
    assert!(!(a > a));
    assert!(!(b > b));

    assert!(a <= a);
    assert!(b <= b);
}

/// Tests PartialOrd for `a` and `b` where `a <= b` is true.
pub fn test_le<T: PartialOrd>(a: T, b: T) {
    assert!(a <= b);
    assert!(b >= a);

    assert!(a == b || a < b);
    assert!(a == b || b > a);

    if a == b {
        assert!(!(a < b));
        assert!(!(b > a));

        assert!(!(a != b));
    } else {
        assert!(a != b);
        test_lt(a, b);
    }
}

/// Test PartialOrd::partial_cmp for `a` and `b` returning `Ordering`
pub fn test_cmp<T: PartialOrd>(a: T, b: T, o: Option<::cmp::Ordering>) {
    assert_eq!(a.partial_cmp(&b), o);
    match o {
        Some(::cmp::Ordering::Less) => {
            test_lt(a, b);
        },
        Some(::cmp::Ordering::Greater) => {
            test_lt(b, a);
        },
        Some(::cmp::Ordering::Equal) => {
            assert!(a == b);
            assert!(!(a != b));
            assert!(!(a < b));
            assert!(!(b < a));
            assert!(!(a > b));
            assert!(!(b > a));
        },
        None => {
            assert!(!(a == b));
            assert!(!(a != b));
            assert!(!(a < b));
            assert!(!(a > b));
            assert!(!(b < a));
            assert!(!(b > a));
            assert!(!(a <= b));
            assert!(!(b <= a));
            assert!(!(a >= b));
            assert!(!(b >= a));
        }
    }
}
