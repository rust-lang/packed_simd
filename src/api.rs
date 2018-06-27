//! Implements the Simd<[T; N]> APIs
//!
//! Each of the APIs is implemented using a macro for each type.
//! The macros live in the sub-modules of the same name.

#[macro_use]
mod cmp;
#[macro_use]
mod fmt;
#[macro_use]
mod minimal_iuf;
#[macro_use]
mod minimal_mask;
#[macro_use]
mod ops;
#[macro_use]
mod reductions;


macro_rules! impl_i {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident
     | $($elem_ids:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_iuf!([$elem_ty; $elem_count]: $tuple_id
                          | $($elem_ids),* | $(#[$doc])*);
        impl_ops_vector_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_scalar_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_vector_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (!(0 as $elem_ty), 0));
        impl_ops_scalar_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (!(0 as $elem_ty), 0));
        impl_ops_vector_shifts!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_scalar_shifts!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_integer_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_bitwise!([$elem_ty; $elem_count]: $tuple_id | (!(0 as $elem_ty), 0));
        impl_fmt!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_partial_eq!([$elem_ty; $elem_count]: $tuple_id | (0, 1));
    }
}

macro_rules! impl_u {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident
     | $($elem_ids:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_iuf!([$elem_ty; $elem_count]: $tuple_id
                          | $($elem_ids),* | $(#[$doc])*);
        impl_ops_vector_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_scalar_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_vector_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (!(0 as $elem_ty), 0));
        impl_ops_scalar_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (!(0 as $elem_ty), 0));
        impl_ops_vector_shifts!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_scalar_shifts!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_integer_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_bitwise!([$elem_ty; $elem_count]: $tuple_id | (!(0 as $elem_ty), 0));
        impl_fmt!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_partial_eq!([$elem_ty; $elem_count]: $tuple_id | (1, 0));
    }
}

macro_rules! impl_f {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident
     | $($elem_ids:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_iuf!([$elem_ty; $elem_count]: $tuple_id
                          | $($elem_ids),* | $(#[$doc])*);
        impl_ops_vector_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_ops_scalar_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_reduction_float_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_partial_eq!([$elem_ty; $elem_count]: $tuple_id | (1., 0.));
    }
}

macro_rules! impl_m {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident | $ielem_ty:ident |
     $($elem_ids:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_mask!([$elem_ty; $elem_count]: $tuple_id | $ielem_ty |
                           $($elem_ids),* | $(#[$doc])*);
        impl_ops_vector_mask_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (true, false));
        impl_ops_scalar_mask_bitwise!([$elem_ty; $elem_count]: $tuple_id  | (true, false));
        impl_reduction_mask!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_partial_eq!([$elem_ty; $elem_count]: $tuple_id | (true, false));
    }
}
