// FIXME: these types are unsound in C FFI already
// See https://github.com/rust-lang/rust/issues/53346
#![allow(improper_ctypes_definitions)]
#![deny(rust_2018_idioms)]
#![cfg_attr(test, feature(avx512_target_feature, abi_vectorcall))]

#[cfg(test)]
mod api;
