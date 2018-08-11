//! Permute elements of a vector with a dynamic vector of indices.

macro_rules! impl_permute_dyn {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Permute vector lanes according to `indices`.
            #[inline]
            pub fn permute_dyn(self, indices: Self) -> Self {
                codegen::permute_dyn::PermuteDyn::permute_dyn(self, indices)
            }
        }

        test_if! {
            $test_tt:
            interpolate_idents! {
                pub mod [$id _permute_dyn] {
                    use super::*;
                    #[cfg_attr(not(target_arch = "wasm32"), test)] #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
                    fn permute_dyn() {
                        let increasing = {
                            let mut v = $id::splat(0);
                            for i in 0..$id::lanes() {
                                v = v.replace(i, i as $elem_ty);
                            }
                            v
                        };
                        let decreasing = {
                            let mut v = $id::splat(0);
                            for i in 0..$id::lanes() {
                                v = v.replace(i, ($id::lanes() - 1 - i) as $elem_ty);
                            }
                            v
                        };

                        assert_eq!(increasing.permute_dyn(increasing), increasing, "(i,i)=>i");
                        assert_eq!(decreasing.permute_dyn(increasing), decreasing, "(d,i)=>d");
                        assert_eq!(increasing.permute_dyn(decreasing), decreasing, "(i,d)=>d");
                        assert_eq!(decreasing.permute_dyn(decreasing), increasing, "(d,d)=>i");

                        for i in 0..$id::lanes() {
                            assert_eq!(increasing.permute_dyn($id::splat(i as $elem_ty)),
                                       $id::splat(increasing.extract(i)));
                            assert_eq!(decreasing.permute_dyn($id::splat(i as $elem_ty)),
                                       $id::splat(decreasing.extract(i)));

                            assert_eq!($id::splat(i as $elem_ty).permute_dyn(increasing),
                                       $id::splat(i as $elem_ty));
                            assert_eq!($id::splat(i as $elem_ty).permute_dyn(decreasing),
                                       $id::splat(i as $elem_ty));
                        }

                    }
                }
            }
        }
    };
}
