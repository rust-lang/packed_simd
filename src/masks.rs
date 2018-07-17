//! Mask types

/// 8-bit wide mask.
#[derive(Copy, Clone)]
pub struct m8(i8);

/// 16-bit wide mask.
#[derive(Copy, Clone)]
pub struct m16(i16);

/// 32-bit wide mask.
#[derive(Copy, Clone)]
pub struct m32(i32);

/// 64-bit wide mask.
#[derive(Copy, Clone)]
pub struct m64(i64);

/// 128-bit wide mask.
#[derive(Copy, Clone)]
pub struct m128(i128);

macro_rules! impl_mask_primitive_traits {
    ($id:ident: $elem_ty:ident) => {
        impl Default for $id {
            #[inline]
            fn default() -> Self {
                $id(0)
            }
        }

        impl PartialEq<$id> for $id {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
            #[inline]
            fn ne(&self, other: &Self) -> bool {
                self.0 != other.0
            }
        }

        impl Eq for $id {}

        impl PartialOrd<$id> for $id {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<crate::cmp::Ordering> {
                // FIXME: add assumes
                use crate::cmp::Ordering;
                if self == other {
                    Some(Ordering::Equal)
                } else if self.0 > other.0 {
                    // Note:
                    //  * false = 0_i
                    //  * true == !0_i == -1_i
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }

            #[inline]
            fn lt(&self, other: &Self) -> bool {
                self.0 > other.0
            }
            #[inline]
            fn gt(&self, other: &Self) -> bool {
                self.0 < other.0
            }
            #[inline]
            fn le(&self, other: &Self) -> bool {
                self.0 >= other.0
            }
            #[inline]
            fn ge(&self, other: &Self) -> bool {
                self.0 <= other.0
            }
        }

        impl Ord for $id {
            #[inline]
            fn cmp(&self, other: &Self) -> crate::cmp::Ordering {
                match self.partial_cmp(other) {
                    Some(x) => x,
                    None => { unsafe { crate::hint::unreachable_unchecked() } }
                }
            }
        }

        impl crate::hash::Hash for $id {
            #[inline]
            fn hash<H: crate::hash::Hasher>(&self, state: &mut H) {
                (self.0 != 0).hash(state);
            }
        }

        impl crate::fmt::Debug for $id {
            #[inline]
            fn fmt(&self, fmtter: &mut crate::fmt::Formatter) -> Result<(), crate::fmt::Error> {
                write!(fmtter, "{}({})", stringify!($id), self.0 != 0)
            }
        }
    }
}

impl_mask_primitive_traits!(m8: i8);
impl_mask_primitive_traits!(m16: i16);
impl_mask_primitive_traits!(m32: i32);
impl_mask_primitive_traits!(m64: i64);
impl_mask_primitive_traits!(m128: i128);
