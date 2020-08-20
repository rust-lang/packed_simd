//! Implements vertical (lane-wise) floating-point `acos`.

macro_rules! impl_math_float_acos {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Arccosine.
            #[inline]
            pub fn acos(self) -> Self {
                use crate::codegen::math::float::acos::Acos;
                Acos::acos(self)
            }
        }

        test_if!{
            $test_tt:
            paste::item! {
                pub mod [<$id _math_acos>] {
                    use super::*;
                    #[cfg_attr(not(target_arch = "wasm32"), test)] #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
                    fn acos() {
                        let z = $id::splat(0 as $elem_ty);
                        let o = $id::splat(1 as $elem_ty);

                        assert_eq!(z, o.acos());
                    }
                }
            }
        }
    };
}
