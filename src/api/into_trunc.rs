//! Implementation of `FromTrunc` and `IntoTrunc`.

/// Truncating conversion from `T` to `Self`.
pub trait FromTrunc<T>: ::marker::Sized {
    /// Truncating conversion from `T` to `Self`.
    fn from_truncated(T) -> Self;
}

/// Truncating conversion from `Self` to `T`.
pub trait IntoTrunc<T>: ::marker::Sized {
    /// Truncating conversion from `self` to `T`.
    fn trunc(self) -> T;
}

/// `FromTrunc` implies `IntoTrunc`.
impl<T, U> IntoTrunc<U> for T where U: FromTrunc<T> {
    #[inline]
    fn trunc(self) -> U {
        U::from_truncated(self)
    }
}

/// `FromTrunc` and `IntoTrunc` are reflexive
impl<T> FromTrunc<T> for T {
    #[inline]
    fn from_truncated(t: Self) -> Self {
        t
    }
}

#[macro_use]
mod macros;

mod v16;
pub use self::v16::*;

mod v32;
pub use self::v32::*;

mod v64;
pub use self::v64::*;

mod v128;
pub use self::v128::*;

mod v256;
pub use self::v256::*;

mod v512;
pub use self::v512::*;
