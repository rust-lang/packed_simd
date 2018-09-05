# `std::simd`'s performance guide

Writing fast and portable SIMD algorithms using `packed_simd` is, unfortunately,
not trivial. There are many pitfals that one should be aware of, and some idioms
that help avoid those pitfalls. This document attempts to document these.

## Table-of-contents

- [Vertical vs Horizontal operations](#vertical-vs-horizontal-operations)

- [target-features](#target-features)
  - [`RUSTFLAGS` (global-features)](#rustflags)
  - [`target_feature` attribute](#attribute)
  - [inlining](#inlining)
  - [run-time feature detection](#runtime-detection)

- [floating-point-math](#floating-point-math)
  - [Short-vector math library](#short-vector-math-library)
  - [Approximate math functions](#approximate-functions)
  - [Fused Multipy Add (FMA)](#fused-multipy-add)

- [Checks](#checks)
  - [Bound checks](#bound-checks)

## Vertical vs Horizontal Operations

The result of vertical operations, like vector negation: `-a`, for a given lane,
does not depend on the result of the operation for the other lanes. The result
of horizontal operations, like the vector `sum` reduction: `a.sum()`, depends on
the value of all vector lanes.

In virtually all architectures vertical operations are fast, while horizontal
operations are, by comparison, very slow.

Consider the following two functions for computing the sum of all `f32` values
in a slice:

```rust
fn fast_sum(x: &[f32]) -> f32 {
    assert!(x.len() % 4 == 0);
    let mut sum = f32x4::splat(0.); // [0., 0., 0., 0.]
    for i in (0..x.len()).step_by(4) {
        sum += f32x4::from_slice_unaligned(&x[i..]);
    }
    sum.sum()
}

fn slow_sum(x: &[f32]) -> f32 {
    assert!(x.len() % 4 == 0);
    let mut sum: f32 = 0.;
    for i in (0..x.len()).step_by(4) {
        sum += f32x4::from_slice_unaligned(&x[i..]).sum();
    }
    sum
}
```

The inner loop over the slice is where the bulk of the work actually happens.
There, the `fast_sum` function perform vertical operations into a vector, doing
a single horizontal reduction at the end, while the `slow_sum` function performs
horizontal vector operations inside of the loop.

On all widely-used architectures, `fast_sum` is a large constant factor faster
than `slow_sum`. You can run the [slice_sum]() example and see for yourself. On
the particular machine tested there the algorithm using the horizontal vector
addition is 2.7x slower than the one using vertical vector operations!

## Enabling target features

Not all processors of a certain architecture will have SIMD processing units,
and using a SIMD instruction which is not supported will trigger undefined behavior.

To allow building safe, portable programs, the Rust compiler will **not**, by default,
generate any sort of vector instructions, unless it can statically determine
they are supported. For example, on AMD64, SSE2 support is architecturally guaranteed.
The `x86_64-apple-darwin` target enables up to SSSE3. The get a defintive list of
which features are enabled by default on various platforms, refer to the target
specifications [in the compiler's source code][targets].

[targets]: https://github.com/rust-lang/rust/tree/master/src/librustc_target/spec

### RUSTFLAGS

One of the easiest ways to benefit from SIMD is to allow the compiler
to generate code using certain vector instruction extensions.

The environment variable `RUSTFLAGS` can be used to pass options for code
generation to the Rust compiler. These flags will affect **all** compiled crates.

There are two flags which can be used to enable specific vector extensions:

#### target-feature

- Syntax: `-C target-feature=<features>`

- Provides the compiler with a comma-separated set of instruction extensions
  to enable.

  **Example**: Use `-C target-features=+sse3,+avx` to enable generating instructions
  for [Streaming SIMD Extensions 3](https://en.wikipedia.org/wiki/SSE3) and
  [Advanced Vector Extensions](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions).

- To list target triples for all targets supported by Rust, use:

  ```sh
  rustc --print target-list
  ```

- To list all support target features for a certain target triple, use:

  ```sh
  rustc --target=${TRIPLE} --print target-features
  ```

- Note that all CPU features are independent, and will have to be enabled individually.

  **Example**: Setting `-C target-features=+avx2` will _not_ enable `fma`, even though
  all CPUs which support AVX2 also support FMA. To enable both, one has to use
  `-C target-features=+avx2,+fma`

- Some features also depend on other features, which need to be enabled for the
  target instructions to be generated.

  **Example**: Unless `v7` is specified as the target CPU (see below), to enable
  NEON on ARM it is necessary to use `-C target-feature=+v7,+neon`.

#### target-cpu

- Syntax: `-C target-cpu=<cpu>`

- Sets the identifier of a CPU family / model for which to build and optimize the code.

  **Example**: `RUSTFLAGS='-C target-cpu=cortex-a75'`

- To list all supported target CPUs for a certain target triple, use:

  ```sh
  rustc --target=${TRIPLE} --print target-cpus
  ```

  **Example**:

  ```sh
  rustc --target=i686-pc-windows-msvc --print target-cpus
  ```

- The compiler will translate this into a list of target features. Therefore,
  individual feature checks (`#[cfg(target_feature = "...")]`) will still
  work properly.

- It will cause the code generator to optimize the generated code for that
  specific CPU model.

- Using `native` as the CPU model will cause Rust to generate and optimize code
  for the CPU running the compiler. It is useful when building programs which you
  plan to only use locally. This should never be used when the generated programs
  are meant to be run on other computers, such as when packaging for distribution
  or cross-compiling.

### Attribute

Explain the `#[target_feature]` attribute

### Inlining

TODO

Explain how the `#[target_feature]` attribute interacts with inlining

### Run-time feature detection

TODO

Explain cost (how it works).

### In practice

Using `RUSTFLAGS` will allow the crate being compiled, as well as all its
transitive dependencies to use certain target features.

A tehnique used to avoid undefined behavior at runtime is to compile and
ship multiple binaries, each compiled with a certain set of features.
This might not be feasible in some cases, and can quickly get out of hand
as more and more vector extensions are added to an architecture.

Rust can be more flexible: you can build a single binary/library which automatically
picks the best supported vector instructions depending on the host machine.
The trick consists of monomorphizing parts of the code during building, and then
using run-time feature detection to select the right code path when running.

<!-- TODO
Explain how to create efficient functions that dispatch to different
implementations at run-time without issues (e.g. using `#[inline(always)]` for
the impls, wrapping in `#[target_feature]`, and the wrapping those in a function
that does run-time feature detection).
-->

**NOTE** (x86 specific): because the AVX (256-bit) registers extend the existing
SSE (128-bit) registers, mixing SSE and AVX instructions in a program can cause
performance issues.

The solution is to compile all code, even the code written with 128-bit vectors,
with the AVX target feature enabled. This will cause the compiler to prefix the
generated instructions with the [VEX] prefix.

[VEX]: https://en.wikipedia.org/wiki/VEX_prefix

## Floating-point math

### Short Vector Math Library

TODO

Explain how is short-vector math performed by default (just scalarized libm calls).

Explain how to enable `sleef`, etc.

### Approximate functions

TODO

Explain that they exists, that they are often _much_ faster, how to use them,
that people should check whether the error is good enough for their
applications. Explain that this error is currently unstable and might change.

### Fused Multiply Add

TODO

Explain that this is a compound operation, infinite precision, difference
between `mul_add` and `mul_adde`, that LLVM cannot do this by itself, etc.

## Checks

### Bound checks

Reading and writing packed vectors to/from slices is checked by default.
Independently of the configuration options used, the safe functions:

* `Simd<[T; N]>::from_slice_aligned(& s[..])`
* `Simd<[T; N]>::write_to_slice_aligned(&mut s[..])`

always check that:

* the slice is big enough to hold the vector
* the slice is suitably aligned to perform an aligned load/store for a `Simd<[T;
  N]>` (this alignment is often much larger than that of `T`).

There are `_unaligned` versions that use unaligned load and stores, as well as
`unsafe` `_unchecked` that do not perform any checks iff `debug-assertions =
false` / `debug = false`. That is, the `_unchecked` methods do still assert size
and alignment in debug builds and could also do so in release builds depending
on the configuration options.

These assertions do often significantly impact performance and you should be
aware of them.
