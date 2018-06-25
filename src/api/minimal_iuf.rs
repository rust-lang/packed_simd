//! Minimal API of signed integer, unsigned integer, and floating-point vectors.

macro_rules! impl_minimal_iuf {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident |
     $($elem_name:ident),+ |
     $(#[$doc:meta])*) => {

        $(#[$doc])*
        pub type $tuple_id = Simd<[$elem_ty; $elem_count]>;

        impl Simd<[$elem_ty; $elem_count]> {
            /// Creates a new instance with each vector elements initialized
            /// with the provided values.
            #[inline]
            pub const fn new($($elem_name: $elem_ty),*) -> Self {
                Simd(sealed::$tuple_id($($elem_name),*))
            }

            /// Returns the number of vector lanes.
            #[inline]
            pub const fn lanes() -> usize {
                $elem_count
            }

            /// Constructs a new instance with each element initialized to
            /// `value`.
            #[inline]
            pub const fn splat(value: $elem_ty) -> Self {
                Simd(sealed::$tuple_id($({
                    #[allow(non_camel_case_types, dead_code)]
                    struct $elem_name;
                    value
                }),*))
            }

            /// Extracts the value at `index`.
            ///
            /// # Panics
            ///
            /// If `index >= Self::lanes()`.
            #[inline]
            pub fn extract(self, index: usize) -> $elem_ty {
                assert!(index < $elem_count);
                unsafe { self.extract_unchecked(index) }
            }

            /// Extracts the value at `index`.
            ///
            /// # Precondition
            ///
            /// If `index >= Self::lanes()` the behavior is undefined.
            #[inline]
            pub unsafe fn extract_unchecked(self, index: usize) -> $elem_ty {
                use llvm::simd_extract;
                simd_extract(self.0, index as u32)
            }

            /// Returns a new vector where the value at `index` is replaced by `new_value`.
            ///
            /// # Panics
            ///
            /// If `index >= Self::lanes()`.
            #[inline]
            #[must_use = "replace does not modify the original value - it returns a new vector with the value at `index` replaced by `new_value`d"]
            pub fn replace(self, index: usize, new_value: $elem_ty) -> Self {
                assert!(index < $elem_count);
                unsafe { self.replace_unchecked(index, new_value) }
            }

            /// Returns a new vector where the value at `index` is replaced by `new_value`.
            ///
            /// # Precondition
            ///
            /// If `index >= Self::lanes()` the behavior is undefined.
            #[inline]
            #[must_use = "replace_unchecked does not modify the original value - it returns a new vector with the value at `index` replaced by `new_value`d"]
            pub unsafe fn replace_unchecked(
                self,
                index: usize,
                new_value: $elem_ty,
            ) -> Self {
                use llvm::simd_insert;
                Simd(simd_insert(self.0, index as u32, new_value))
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [test_minimal_ $tuple_id] {
                use super::*;
                #[test]
                fn minimal() {
                    // lanes:
                    assert_eq!($elem_count, $tuple_id::lanes());

                    // splat and extract / extract_unchecked:
                    const VAL: $elem_ty = 7 as $elem_ty;
                    const VEC: $tuple_id = $tuple_id::splat(VAL);
                    for i in 0..$tuple_id::lanes() {
                        assert_eq!(VAL, VEC.extract(i));
                        assert_eq!(VAL, unsafe { VEC.extract_unchecked(i) });
                    }

                    // replace / replace_unchecked
                    let new_vec = VEC.replace(1, 42 as $elem_ty);
                    for i in 0..$tuple_id::lanes() {
                        if i == 1 {
                            assert_eq!(42 as $elem_ty, new_vec.extract(i));
                        } else {
                            assert_eq!(VAL, new_vec.extract(i));
                        }
                    }
                    let new_vec = unsafe { VEC.replace_unchecked(1, 42 as $elem_ty) };
                    for i in 0..$tuple_id::lanes() {
                        if i == 1 {
                            assert_eq!(42 as $elem_ty, new_vec.extract(i));
                        } else {
                            assert_eq!(VAL, new_vec.extract(i));
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn minimal_extract_panic_on_out_of_bounds() {
                    const VAL: $elem_ty = 7 as $elem_ty;
                    const VEC: $tuple_id = $tuple_id::splat(VAL);
                    let _ = VEC.extract($tuple_id::lanes());
                }
                #[test]
                #[should_panic]
                fn minimal_replace_panic_on_out_of_bounds() {
                    const VAL: $elem_ty = 7 as $elem_ty;
                    const VEC: $tuple_id = $tuple_id::splat(VAL);
                    let _ = VEC.replace($tuple_id::lanes(), 42 as $elem_ty);
                }
            }
        }
    }
}
