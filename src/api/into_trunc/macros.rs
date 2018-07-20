//! Macros implementing `FromTrunc`

macro_rules! impl_from_trunc_ {
    ($id:ident: $from_ty:ident) => {
        impl crate::api::into_trunc::FromTrunc<$from_ty> for $id {
            #[inline]
            fn from_truncated(x: $from_ty) -> Self {
                use ::codegen::llvm::simd_cast;
                debug_assert_eq!($from_ty::lanes(), $id::lanes());
                Simd(unsafe { simd_cast(x.0) })
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _from_trunc_ $from_ty] {
                use super::*;
                #[test]
                fn test() {
                    assert_eq!($id::lanes(), $from_ty::lanes());
                }
            }
        }
    }
}

macro_rules! impl_from_trunc {
    ($id:ident: $($from_ty:ident),*) => {
        $(
            impl_from_trunc_!($id: $from_ty);
        )*
    }
}

macro_rules! impl_from_trunc_mask_ {
    ($id:ident: $from_ty:ident) => {
        impl crate::api::into_trunc::FromTrunc<$from_ty> for $id {
            #[inline]
            fn from_truncated(x: $from_ty) -> Self {
                debug_assert_eq!($from_ty::lanes(), $id::lanes());
                x.ne($from_ty::default()).select($id::splat(true), $id::splat(false))
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _from_trunc_ $from_ty] {
                use super::*;
                #[test]
                fn test() {
                    assert_eq!($id::lanes(), $from_ty::lanes());

                    let x = $from_ty::default();
                    let m: $id = x.into_trunc();
                    assert!(m.none());
                }
            }
        }
    }
}

macro_rules! impl_from_trunc_mask {
    ($id:ident: $($from_ty:ident),*) => {
        $(
            impl_from_trunc_mask_!($id: $from_ty);
        )*
    }
}


#[allow(unused)]
macro_rules! impl_into_trunc {
    ($id:ident: $($from_ty:ident),*) => {
        $(
            impl_from_trunc_!($from_ty: $id);
        )*
    }
}
