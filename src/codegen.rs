//! Code-generation utilities

crate mod reductions;

macro_rules! impl_simd_array {
    ([$elem_ty:ident; $elem_cnt:expr]:
     $tuple_id:ident | $($elem_tys:ident),*) => {
        #[derive(Copy, Clone)]
        #[repr(simd)]
        pub struct $tuple_id($(crate $elem_tys),*);
        //^^^^^^^ leaked through SimdArray

        impl crate::sealed::SimdArray for [$elem_ty; $elem_cnt] {
            type Tuple = $tuple_id;
            type T = $elem_ty;
            const N: usize = $elem_cnt;
        }
    }
}

crate mod v128;
crate use self::v128::*;

crate mod v256;
crate use self::v256::*;
