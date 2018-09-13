#![cfg(test)]
#![deny(warnings, rust_2018_idioms)]
#![feature(plugin, avx512_target_feature, abi_vectorcall)]
#![plugin(interpolate_idents)]

use cfg_if::cfg_if;

mod api;

cfg_if! {
    if #[cfg(debug_assertions)] {
        compile_error!("the verify tests only run in --release mode");
    }
}
