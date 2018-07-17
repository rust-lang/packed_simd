//! Implements methods to read a vector type from a slice.

macro_rules! impl_slice_from_slice {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl $id {
            /// Instantiates a new vector with the values of the `slice`.
            ///
            /// # Panics
            ///
            /// If `slice.len() < Self::lanes()` or `&slice[0]` is not aligned
            /// to an `align_of::<Self>()` boundary.
            #[inline]
            pub fn from_slice_aligned(slice: &[$elem_ty]) -> Self {
                unsafe {
                    assert!(slice.len() >= $elem_count);
                    let target_ptr = slice.get_unchecked(0) as *const $elem_ty;
                    assert!(
                        target_ptr.align_offset(::mem::align_of::<Self>())
                            == 0
                    );
                    Self::from_slice_aligned_unchecked(slice)
                }
            }

            /// Instantiates a new vector with the values of the `slice`.
            ///
            /// # Panics
            ///
            /// If `slice.len() < Self::lanes()`.
            #[inline]
            pub fn from_slice_unaligned(slice: &[$elem_ty]) -> Self {
                unsafe {
                    assert!(slice.len() >= $elem_count);
                    Self::from_slice_unaligned_unchecked(slice)
                }
            }

            /// Instantiates a new vector with the values of the `slice`.
            ///
            /// # Precondition
            ///
            /// If `slice.len() < Self::lanes()` or `&slice[0]` is not aligned
            /// to an `align_of::<Self>()` boundary, the behavior is undefined.
            #[inline]
            pub unsafe fn from_slice_aligned_unchecked(slice: &[$elem_ty]) -> Self {
                #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
                *(slice.get_unchecked(0) as *const $elem_ty as *const Self)
            }

            /// Instantiates a new vector with the values of the `slice`.
            ///
            /// # Precondition
            ///
            /// If `slice.len() < Self::lanes()` the behavior is undefined.
            #[inline]
            pub unsafe fn from_slice_unaligned_unchecked(
                slice: &[$elem_ty],
            ) -> Self {
                use mem::size_of;
                let target_ptr =
                    slice.get_unchecked(0) as *const $elem_ty as *const u8;
                let mut x = Self::splat(0 as $elem_ty);
                let self_ptr = &mut x as *mut Self as *mut u8;
                ptr::copy_nonoverlapping(
                    target_ptr,
                    self_ptr,
                    size_of::<Self>(),
                );
                x
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _slice_from_slice] {
                use super::*;
                use iter::Iterator;

                #[test]
                fn from_slice_unaligned() {
                    let mut unaligned = [42 as $elem_ty; $id::lanes() + 1];
                    unaligned[0] = 0 as $elem_ty;
                    let vec = $id::from_slice_unaligned(&unaligned[1..]);
                    for (index, &b) in unaligned.iter().enumerate() {
                        if index == 0 {
                            assert_eq!(b, 0 as $elem_ty);
                        } else {
                            assert_eq!(b, vec.extract(index - 1));
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn from_slice_unaligned_fail() {
                    let mut unaligned = [42 as $elem_ty; $id::lanes() + 1];
                    unaligned[0] = 0 as $elem_ty;
                    let _vec = $id::from_slice_unaligned(&unaligned[2..]);
                }

                union A {
                    data: [$elem_ty; 2 * $id::lanes()],
                    _vec: $id,
                }

                #[test]
                fn from_slice_aligned() {
                    let mut aligned = A {
                        data: [0 as $elem_ty; 2 * $id::lanes()],
                    };
                    for i in $id::lanes()..(2 * $id::lanes()) {
                        unsafe {
                            aligned.data[[i]] = 42 as $elem_ty;
                        }
                    }

                    let vec =
                        unsafe { $id::from_slice_aligned(&aligned.data[$id::lanes()..]) };
                    for (index, &b) in unsafe { aligned.data.iter().enumerate() } {
                        if index < $id::lanes() {
                            assert_eq!(b, 0 as $elem_ty);
                        } else {
                            assert_eq!(b, vec.extract(index - $id::lanes()));
                        }
                    }
                }

                #[test]
                #[should_panic]
                fn from_slice_aligned_fail_lanes() {
                    let aligned = A {
                        data: [0 as $elem_ty; 2 * $id::lanes()],
                    };
                    let _vec = unsafe {
                        $id::from_slice_aligned(&aligned.data[2 * $id::lanes()..])
                    };
                }

                #[test]
                #[should_panic]
                fn from_slice_aligned_fail_align() {
                    unsafe {
                        let aligned = A {
                            data: [0 as $elem_ty; 2 * $id::lanes()],
                        };
                        // offset the aligned data by one byte:
                        let s: &[u8; 2
                                 * $id::lanes()
                                 * mem::size_of::<$elem_ty>()] =
                            mem::transmute(&aligned.data);
                        let s: &[[$elem_ty]] = slice::from_raw_parts(
                            s.get_unchecked(1) as *const u8 as *const $elem_ty,
                            $id::lanes(),
                        );
                        let _vec = $id::from_slice_aligned(s);
                    }
                }
            }
        }
    };
}
