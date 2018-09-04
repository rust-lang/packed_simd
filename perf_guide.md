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

### RUSTFLAGS

TODO

Enabling features for the whole binary using `-C target-feature` and `-C target-cpu`, ...

Arch specific information: e.g. `-C target-feature=+avx2` does not enable `fma` 

### Attribute

TODO

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
