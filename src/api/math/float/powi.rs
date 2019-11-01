//! Implements vertical (lane-wise) floating-point `powi`.

macro_rules! impl_math_float_powi {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Raises `self` number to the integer power of `x`.
            #[inline]
            pub fn powi(self, x: i32) -> Self {
                use crate::codegen::math::float::powi::Powi;
                Powi::powi(self, x)
            }
        }

        test_if! {
            $test_tt:
            paste::item! {
                pub mod [<$id _math_powi>] {
                    use super::*;
                    #[cfg_attr(not(target_arch = "wasm32"), test)]
                    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
                    fn powf() {
                        let z = $id::splat(0 as $elem_ty);
                        let o = $id::splat(1 as $elem_ty);
                        let t = $id::splat(2 as $elem_ty);
                        let f = $id::splat(4 as $elem_ty);
                        let s = $id::splat(7 as $elem_ty);
                        let e = $id::splat(8 as $elem_ty);
                        assert_eq!(z, z.powi(1));
                        assert_eq!(o, z.powi(0));
                        assert_eq!(o, o.powi(0));
                        assert_eq!(o, t.powi(0));
                        assert_eq!(o, o.powi(1));
                        assert_eq!(t, t.powi(1));
                        assert_eq!(f, t.powi(2));
                        assert_eq!(e, t.powi(3));
                        assert_eq!($id::splat(16807 as $elem_ty), s.powi(5));
                    }
                }
            }
        }
    };
}
