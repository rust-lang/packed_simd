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
mod ops_vector_arithmetic;

macro_rules! impl_i {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident
     | $($elem_ids:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_iuf!([$elem_ty; $elem_count]: $tuple_id
                          | $($elem_ids),* | $(#[$doc])*);
        impl_ops_vector_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
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
        impl_fmt!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_partial_eq!([$elem_ty; $elem_count]: $tuple_id | (0, 1));
    }
}

macro_rules! impl_f {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident
     | $($elem_ids:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_iuf!([$elem_ty; $elem_count]: $tuple_id
                          | $($elem_ids),* | $(#[$doc])*);
        impl_ops_vector_arithmetic!([$elem_ty; $elem_count]: $tuple_id);
        impl_fmt!([$elem_ty; $elem_count]: $tuple_id);
        impl_cmp_partial_eq!([$elem_ty; $elem_count]: $tuple_id | (0., 1.));
    }
}
