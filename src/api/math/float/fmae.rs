//! Implements vertical (lane-wise) floating-point `fmae`.

macro_rules! impl_math_float_fmae {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Fused multiply add estimate: ~= `self * y + z`
            ///
            /// While fused multiply-add (`fma`) has infinite precision,
            /// `fmae` has at worst the same precision of a multiply followed by an add.
            /// This might be more efficient on architectures that do not have an `fma` instruction.
            #[inline]
            pub fn fmae(self, y: Self, z: Self) -> Self {
                use crate::codegen::math::float::fmae::Fmae;
                Fmae::fmae(self, y, z)
            }
        }

        test_if!{
            $test_tt:
            interpolate_idents! {
                pub mod [$id _math_fmae] {
                    use super::*;
                    #[cfg_attr(not(target_arch = "wasm32"), test)] #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
                    fn fmae() {
                        let z = $id::splat(0 as $elem_ty);
                        let o = $id::splat(1 as $elem_ty);
                        let t = $id::splat(2 as $elem_ty);
                        let t3 = $id::splat(3 as $elem_ty);
                        let f = $id::splat(4 as $elem_ty);

                        assert_eq!(z, z.fmae(z, z));
                        assert_eq!(o, o.fmae(o, z));
                        assert_eq!(o, o.fmae(z, o));
                        assert_eq!(o, z.fmae(o, o));

                        assert_eq!(t, o.fmae(o, o));
                        assert_eq!(t, o.fmae(t, z));
                        assert_eq!(t, t.fmae(o, z));

                        assert_eq!(f, t.fmae(t, z));
                        assert_eq!(f, t.fmae(o, t));
                        assert_eq!(t3, t.fmae(o, o));
                    }
                }
            }
        }
    };
}
