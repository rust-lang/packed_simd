//! Implements vertical (lane-wise) floating-point `sqrte`.

macro_rules! impl_math_float_sqrte {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl $id {
            #[inline]
            pub fn sqrte(self) -> Self {
                use llvm::simd_fsqrt;
                Simd(unsafe { simd_fsqrt(self.0) })
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _math_sqrte] {
                use super::*;
                #[test]
                fn sqrte() {
                    use $elem_ty::consts::SQRT_2;
                    let z = $id::splat(0 as $elem_ty);
                    let o = $id::splat(1 as $elem_ty);
                    assert_eq!(z, z.sqrte());
                    assert_eq!(o, o.sqrte());

                    let t = $id::splat(2 as $elem_ty);
                    let e = $id::splat(SQRT_2 as $elem_ty);
                    let error = (e - t.sqrte()).abs();
                    let tol = $id::splat(2.4e-4 as $elem_ty);

                    assert!(error.le(tol).all());
                }
            }
        }
    };
}
