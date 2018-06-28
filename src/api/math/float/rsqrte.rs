//! Implements vertical (lane-wise) floating-point `rsqrte`.

macro_rules! impl_math_float_rsqrte {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl $id {
            #[inline]
            pub fn rsqrte(self) -> Self {
                unsafe {
                    use llvm::simd_fsqrt;
                    $id::splat(1.) / Simd(simd_fsqrt(self.0))
                }
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _math_rsqrte] {
                use super::*;
                #[test]
                fn rsqrte() {
                    use $elem_ty::consts::SQRT_2;
                    let o = $id::splat(1 as $elem_ty);
                    assert_eq!(o, o.rsqrte());

                    let t = $id::splat(2 as $elem_ty);
                    let e = 1. / SQRT_2;
                    let error = (e - t.rsqrte()).abs();
                    let tol = $id::splat(2.4e-4 as $elem_ty);
                    assert!(error.le(tol).all());
                }
            }
        }
    };
}
