//! Implements methods to write a vector type to a slice.

macro_rules! impl_slice_write_to_slice {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl $id {
            /// Writes the values of the vector to the `slice`.
            ///
            /// # Panics
            ///
            /// If `slice.len() < Self::lanes()` or `&slice[0]` is not
            /// aligned to an `align_of::<Self>()` boundary.
            #[inline]
            pub fn write_to_slice_aligned(self, slice: &mut [$elem_ty]) {
                unsafe {
                    assert!(slice.len() >= $elem_count);
                    let target_ptr =
                        slice.get_unchecked_mut(0) as *mut $elem_ty;
                    assert!(
                        target_ptr.align_offset(mem::align_of::<Self>())
                            == 0
                    );
                    self.write_to_slice_aligned_unchecked(slice);
                }
            }

            /// Writes the values of the vector to the `slice`.
            ///
            /// # Panics
            ///
            /// If `slice.len() < Self::lanes()`.
            #[inline]
            pub fn write_to_slice_unaligned(self, slice: &mut [$elem_ty]) {
                unsafe {
                    assert!(slice.len() >= $elem_count);
                    self.write_to_slice_unaligned_unchecked(slice);
                }
            }

            /// Writes the values of the vector to the `slice`.
            ///
            /// # Precondition
            ///
            /// If `slice.len() < Self::lanes()` or `&slice[0]` is not
            /// aligned to an `align_of::<Self>()` boundary, the behavior is
            /// undefined.
            #[inline]
            pub unsafe fn write_to_slice_aligned_unchecked(
                self, slice: &mut [$elem_ty],
            ) {
                *(slice.get_unchecked_mut(0) as *mut $elem_ty as *mut Self) =
                    self;
            }

            /// Writes the values of the vector to the `slice`.
            ///
            /// # Precondition
            ///
            /// If `slice.len() < Self::lanes()` the behavior is undefined.
            #[inline]
            pub unsafe fn write_to_slice_unaligned_unchecked(
                self, slice: &mut [$elem_ty],
            ) {
                let target_ptr =
                    slice.get_unchecked_mut(0) as *mut $elem_ty as *mut u8;
                let self_ptr = &self as *const Self as *const u8;
                ptr::copy_nonoverlapping(
                    self_ptr,
                    target_ptr,
                    mem::size_of::<Self>(),
                );
            }
        }


        #[cfg(test)]
        interpolate_idents! {
            mod [$id _slice_write_to_slice] {
                use super::*;
                use iter::Iterator;

                #[test]
                fn write_to_slice_unaligned() {
                    let mut unaligned = [0 as $elem_ty; $id::lanes() + 1];
                    let vec = $id::splat(42 as $elem_ty);
                    vec.write_to_slice_unaligned(&mut unaligned[1..]);
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
                fn write_to_slice_unaligned_fail() {
                    let mut unaligned = [0 as $elem_ty; $id::lanes() + 1];
                    let vec = $id::splat(42 as $elem_ty);
                    vec.write_to_slice_unaligned(&mut unaligned[2..]);
                }

                                union A {
                    data: [$elem_ty; 2 * $id::lanes()],
                    _vec: $id,
                }

                #[test]
                fn write_to_slice_aligned() {
                    let mut aligned = A {
                        data: [0 as $elem_ty; 2 * $id::lanes()],
                    };
                    let vec = $id::splat(42 as $elem_ty);
                    unsafe { vec.write_to_slice_aligned(&mut aligned.data[$id::lanes()..]) };
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
                fn write_to_slice_aligned_fail_lanes() {
                    let mut aligned = A {
                        data: [0 as $elem_ty; 2 * $id::lanes()],
                    };
                    let vec = $id::splat(42 as $elem_ty);
                    unsafe {
                        vec.write_to_slice_aligned(&mut aligned.data[2 * $id::lanes()..])
                    };
                }

                #[test]
                #[should_panic]
                fn write_to_slice_aligned_fail_align() {
                    unsafe {
                        let mut aligned = A {
                            data: [0 as $elem_ty; 2 * $id::lanes()],
                        };
                        // offset the aligned data by one byte:
                        let s: &mut [u8; 2
                                     * $id::lanes()
                                     * mem::size_of::<$elem_ty>()] =
                            mem::transmute(&mut aligned.data);
                        let s: &mut [[$elem_ty]] = slice::from_raw_parts_mut(
                            s.get_unchecked_mut(1) as *mut u8 as *mut $elem_ty,
                            $id::lanes(),
                        );
                        let vec = $id::splat(42 as $elem_ty);
                        vec.write_to_slice_aligned(s);
                    }
                }

            }
        }
    };
}
