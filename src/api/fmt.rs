//! Implements formatting APIs

#[macro_use]
mod debug;

macro_rules! impl_fmt {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl_fmt_debug!([$elem_ty; $elem_count]: $id);
    };
}
