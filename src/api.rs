//! Implements the Simd<[T; N]> APIs

#[macro_use]
mod cmp;
#[macro_use]
mod default;
#[macro_use]
mod fmt;
#[macro_use]
mod from;
#[macro_use]
mod hash;
#[macro_use]
mod math;
#[macro_use]
mod minimal;
#[macro_use]
mod ops;
#[macro_use]
mod reductions;
#[macro_use]
mod select;
#[macro_use]
mod shuffle;
#[macro_use]
mod slice;
#[macro_use]
mod swap_bytes;

macro_rules! impl_i {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident, $mask_ty:ident
     | $($elem_ids:ident),* | From: $($from_vec_ty:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_iuf!([$elem_ty; $elem_count]: $tuple_id
                          | $($elem_ids),* | $(#[$doc])*);
        impl_ops_vector_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_scalar_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_vector_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (!(0 as $elem_ty), 0));
        impl_ops_scalar_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (!(0 as $elem_ty), 0));
        impl_ops_vector_shifts!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_scalar_shifts!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_vector_neg!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_vector_int_min_max!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_integer_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_min_max!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_bitwise!([$elem_ty; $elem_count]: $tuple_id | $elem_ty | (|x|{ x }) | (!(0 as $elem_ty), 0));
        impl_fmt_debug!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt_lower_hex!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt_upper_hex!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt_octal!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt_binary!([$elem_ty; $elem_count]: $tuple_id);
        impl_from_array!([$elem_ty; $elem_count]: $tuple_id | (1, 1));
        impl_from_vectors!([$elem_ty; $elem_count]: $tuple_id | $($from_vec_ty),*);
        impl_default!([$elem_ty; $elem_count]: $tuple_id);
        impl_hash!([$elem_ty; $elem_count]: $tuple_id);
        impl_slice_from_slice!([$elem_ty; $elem_count]: $tuple_id);
        impl_slice_write_to_slice!([$elem_ty; $elem_count]: $tuple_id);
        impl_swap_bytes!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_partial_eq!([$elem_ty; $elem_count]: $tuple_id | (0, 1));
        impl_cmp_eq!([$elem_ty; $elem_count]: $tuple_id | (0, 1));
        impl_cmp_vertical!([$elem_ty; $elem_count]: $tuple_id, $mask_ty, false, (1, 0));
        impl_cmp_partial_ord!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_ord!([$elem_ty; $elem_count]: $tuple_id | (0, 1));

        test_select!($elem_ty, $mask_ty, $tuple_id, (1, 2));
        // FIXME: test vector shuffles
        // test_shuffle!([$elem_ty; $elem_count]: $tuple_id);
        test_cmp_partial_ord_int!([$elem_ty; $elem_count]: $tuple_id);
    }
}

macro_rules! impl_u {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident, $mask_ty:ident
     | $($elem_ids:ident),* | From: $($from_vec_ty:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_iuf!([$elem_ty; $elem_count]: $tuple_id
                          | $($elem_ids),* | $(#[$doc])*);
        impl_ops_vector_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_scalar_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_vector_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (!(0 as $elem_ty), 0));
        impl_ops_scalar_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (!(0 as $elem_ty), 0));
        impl_ops_vector_shifts!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_scalar_shifts!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_vector_int_min_max!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_integer_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_min_max!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_bitwise!([$elem_ty; $elem_count]: $tuple_id | $elem_ty |
                                (|x|{ x }) | (!(0 as $elem_ty), 0));
        impl_fmt_debug!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt_lower_hex!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt_upper_hex!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt_octal!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt_binary!([$elem_ty; $elem_count]: $tuple_id);
        impl_from_array!([$elem_ty; $elem_count]: $tuple_id | (1, 1));
        impl_from_vectors!([$elem_ty; $elem_count]: $tuple_id | $($from_vec_ty),*);
        impl_default!([$elem_ty; $elem_count]: $tuple_id);
        impl_hash!([$elem_ty; $elem_count]: $tuple_id);
        impl_slice_from_slice!([$elem_ty; $elem_count]: $tuple_id);
        impl_slice_write_to_slice!([$elem_ty; $elem_count]: $tuple_id);
        impl_swap_bytes!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_partial_eq!([$elem_ty; $elem_count]: $tuple_id | (1, 0));
        impl_cmp_eq!([$elem_ty; $elem_count]: $tuple_id | (0, 1));
        impl_cmp_vertical!([$elem_ty; $elem_count]: $tuple_id, $mask_ty, false, (1, 0));
        impl_cmp_partial_ord!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_ord!([$elem_ty; $elem_count]: $tuple_id | (0, 1));

        test_select!($elem_ty, $mask_ty, $tuple_id, (1, 2));

        // FIXME: test vector shuffles
        // test_shuffle!([$elem_ty; $elem_count]: $tuple_id);
        test_cmp_partial_ord_int!([$elem_ty; $elem_count]: $tuple_id);
    }
}

macro_rules! impl_f {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident, $mask_ty:ident
     | $($elem_ids:ident),* | From: $($from_vec_ty:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_iuf!([$elem_ty; $elem_count]: $tuple_id
                          | $($elem_ids),* | $(#[$doc])*);
        impl_ops_vector_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_scalar_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_vector_neg!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_vector_float_min_max!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_float_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_min_max!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt_debug!([$elem_ty; $elem_count]: $tuple_id);
        impl_from_array!([$elem_ty; $elem_count]: $tuple_id | (1., 1.));
        impl_from_vectors!([$elem_ty; $elem_count]: $tuple_id | $($from_vec_ty),*);
        impl_default!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_partial_eq!([$elem_ty; $elem_count]: $tuple_id | (1., 0.));
        impl_slice_from_slice!([$elem_ty; $elem_count]: $tuple_id);
        impl_slice_write_to_slice!([$elem_ty; $elem_count]: $tuple_id);

        // floating-point math
        impl_math_float_abs!([$elem_ty; $elem_count]: $tuple_id);
        impl_math_float_cos!([$elem_ty; $elem_count]: $tuple_id);
        impl_math_float_fma!([$elem_ty; $elem_count]: $tuple_id);
        impl_math_float_rsqrte!([$elem_ty; $elem_count]: $tuple_id);
        impl_math_float_sin!([$elem_ty; $elem_count]: $tuple_id);
        impl_math_float_sqrt!([$elem_ty; $elem_count]: $tuple_id);
        impl_math_float_sqrte!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_vertical!([$elem_ty; $elem_count]: $tuple_id, $mask_ty, false, (1., 0.));

        test_select!($elem_ty, $mask_ty, $tuple_id, (1., 2.));
        test_reduction_float_min_max!([$elem_ty; $elem_count]:$tuple_id);

        // FIXME: test vector shuffles
        // test_shuffle!([$elem_ty; $elem_count]: $tuple_id);
    }
}

macro_rules! impl_m {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident | $ielem_ty:ident |
     $($elem_ids:ident),* | From: $($from_vec_ty:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_mask!([$elem_ty; $elem_count]: $tuple_id | $ielem_ty |
                           $($elem_ids),* | $(#[$doc])*);
        impl_ops_vector_mask_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (true, false));
        impl_ops_scalar_mask_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (true, false));
        impl_reduction_bitwise!([bool; $elem_count]: $tuple_id | $ielem_ty | (|x|{ x != 0 }) | (true, false));
        impl_reduction_mask!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt_debug!([bool; $elem_count]: $tuple_id);
        impl_from_array!([$elem_ty; $elem_count]: $tuple_id | ($elem_ty::new(true), true));
        impl_from_vectors!([$elem_ty; $elem_count]: $tuple_id | $($from_vec_ty),*);
        impl_default!([bool; $elem_count]: $tuple_id);
        impl_cmp_partial_eq!([$elem_ty; $elem_count]: $tuple_id | (true, false));
        impl_cmp_eq!([$elem_ty; $elem_count]: $tuple_id | (true, false));
        impl_cmp_vertical!([$elem_ty; $elem_count]: $tuple_id, $tuple_id, true, (true, false));
        impl_select!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_partial_ord!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_ord!([$elem_ty; $elem_count]: $tuple_id | (false, true));

        // FIXME: test vector shuffles
        // FIXME: test select on masks
        test_cmp_partial_ord_mask!([$elem_ty; $elem_count]: $tuple_id);
    }
}
