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

## Target-features

Not all processors of a certain architecture will have SIMD processing units,
and using a SIMD instruction when it's not supported will trigger a CPU exception.

To allow building safe, portable programs, the Rust compiler will **not**, by default,
generate any sort of vector instructions, unless it can statically determine
they are supported (for example, on AMD64, SSE2 support is architecturally guaranteed).

### RUSTFLAGS

One of the easiest ways to benefit from SIMD is to allow the compiler
to generate code using certain vector instruction extensions.

The environment variable `RUSTFLAGS` can be used to pass options for code
generation to the Rust compiler. These flags will affect **all** compiled crates.

There are two flags which can be used to enable specific vector extensions:

- `-C target-feature=<features>`: a comma-separated set of instruction extensions
  to enable.
  - **NOTE** (x86 specific): the `avx2` feature will _not_ enable `fma`, even though
  all CPUs which support AVX2 also support FMA. You can enable this feature in addition to,
  or separately from AVX2.

<!-- TODO: list all target features for each platform -->

- `-C target-cpu=<cpu>`: the identifier of a CPU family / model for which to build
  and optimize the code.
  - The compiler will translate this into a list of target features.
  - It will also cause the code generator to optimize the generated code
  for that specific CPU model.

<!-- TODO: list all target CPUs for each platform -->

### Attribute

`RUSTFLAGS` is a powerful tool but its extent is too wide. In some cases it might
make sense to ship multiple binaries or (if the target hardware is known) build
an optimized binary only for that specific CPU architecture.

Rust can be more flexible: you can build a single binary/library which automatically
picks the best supported vector instructions depending on the host machine.
We need a way to monomorphize parts of our code during building, and then a way to
select the right code path when running.

Explain the `#[target_feature]` attribute

### Inlining

TODO

Explain how the `#[target_feature]` attribute interacts with inlining

### Run-time feature detection

TODO

Explain cost (how it works).

Explain how to create efficient functions that dispatch to different
implementations at run-time without issues (e.g. using `#[inline(always)]` for
the impls, wrapping in `#[target_feature]`, and the wrapping those in a function
that does run-time feature detection).

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
