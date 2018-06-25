#[macro_use]
mod minimal_iuf;

macro_rules! impl_i {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident
     | $($elem_ids:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_iuf!([$elem_ty; $elem_count]: $tuple_id
                          | $($elem_ids),* | $(#[$doc])*);
    }
}

macro_rules! impl_u {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident
     | $($elem_ids:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_iuf!([$elem_ty; $elem_count]: $tuple_id
                          | $($elem_ids),* | $(#[$doc])*);
    }
}

macro_rules! impl_f {
    ([$elem_ty:ident; $elem_count:expr]: $tuple_id:ident
     | $($elem_ids:ident),* | $(#[$doc:meta])*) => {
        impl_minimal_iuf!([$elem_ty; $elem_count]: $tuple_id
                          | $($elem_ids),* | $(#[$doc])*);
    }
}
