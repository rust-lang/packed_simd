# Enabling target features

Not all processors of a certain architecture will have SIMD processing units,
and using a SIMD instruction which is not supported will trigger undefined behavior.

To allow building safe, portable programs, the Rust compiler will **not**, by default,
generate any sort of vector instructions, unless it can statically determine
they are supported. For example, on AMD64, SSE2 support is architecturally guaranteed.
The get a defintive list of which features are enabled by default on various platforms, refer to the target
specifications [in the compiler's source code][targets]. Youc an also determine the default vector instructiosn with
`rustc --print cfg` for the current target or `rustc --print cfg --target={target}` for a different target.

[targets]: https://github.com/rust-lang/rust/tree/master/src/librustc_target/spec
