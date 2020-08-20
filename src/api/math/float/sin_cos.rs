//! Implements vertical (lane-wise) floating-point `sin_cos` (fused sine and
//! cosine).

macro_rules! impl_math_float_sin_cos {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Sine and cosine of `self`
            #[inline]
            pub fn sin_cos(self) -> (Self, Self) {
                use crate::codegen::math::float::sin_cos::SinCos;
                SinCos::sin_cos(self)
            }

            /// Sine and cosine of `self * PI`.
            #[inline]
            pub fn sin_cos_pi(self) -> (Self, Self) {
                use crate::codegen::math::float::sin_cos_pi::SinCosPi;
                SinCosPi::sin_cos_pi(self)
            }
        }

        test_if!{
            $test_tt:
            paste::item! {
                pub mod [<$id _math_sin_cos>] {
                    use super::*;
                    #[cfg_attr(not(target_arch = "wasm32"), test)] #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
                    fn sin_cos() {
                        use crate::$elem_ty::consts::PI;
                        let z = $id::splat(0 as $elem_ty);
                        let o = $id::splat(1 as $elem_ty);
                        let p = $id::splat(PI as $elem_ty);
                        let ph = $id::splat(PI as $elem_ty / 2.);
                        let (o_r_s, o_r_c) = ($id::splat((PI as $elem_ty / 2.).sin()), $id::splat((PI as $elem_ty / 2.).cos()));
                        let (z_r_s, z_r_c) = ($id::splat((PI as $elem_ty).sin()), $id::splat((PI as $elem_ty).cos()));

                        let (z_s, z_c) = z.sin_cos();
                        let (ph_s, ph_c) = ph.sin_cos();
                        let (p_s, p_c) = p.sin_cos();
                        assert_eq!(z, z_s); // sin(0) = 0
                        assert_eq!(o, z_c); // cos(0) = 1
                        assert_eq!(o_r_s, ph.sin());
                        assert_eq!(o_r_c, ph.cos());
                        assert_eq!(z_r_s, p.sin());
                        assert_eq!(z_r_c, p.cos());
                    }
                }
            }
        }
    };
}
