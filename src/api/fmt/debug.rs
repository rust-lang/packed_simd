//! Implement debug formatting

macro_rules! impl_fmt_debug {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident) => {
        impl ::fmt::Debug for $id {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                write!(f, "{}(", stringify!($id))?;
                for i in 0..$elem_count {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    self.extract(i).fmt(f)?;
                }
                write!(f, ")")
            }
        }
    };
}
