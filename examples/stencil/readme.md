# Stencil

This is the generic [`stencil` ISPC benchmark][ispc]. 

## Usage

```
cargo run --release --features=ispc
```

will run all benchmarks including the ISPC ones. 


## Results

On a dual core AVX1 i5 @1.8 GHz:

| 800 x 600    | time [ms] <br> Rust | speedup [-] <br> Rust-vs-Rust | time [ms] <br> ISPC 1.9.2 | speedup [-] <br> ISPC-vs-Rust |
|--------------|---------------------|-------------------------------|---------------------------|-------------------------------|
| `scalar`     |                2346 |                          1.0x |                         - |                             - |
| `vector`     |                 488 |                          4.8x |                       406 |                          1.2x |
| `vector_par` |                 301 |                          7.8x |                       475 |                          0.6x |

[ispc]: https://github.com/ispc/ispc/tree/master/examples/stencil
