# Mandelbrot

This is the [`mandelbrot` benchmark from the benchmarksgame][bg]. 

## Background

http://mathworld.wolfram.com/MandelbrotSet.html

## Usage

It takes four arguments in this order:

* `width`: width of the image to render
* (optional) `height`: height of the image to render - defaults to `width`
* (optional) `algorithm`: algorithm to use - defaults to the fastest one.
  * `0`: scalar algorithm
  * `1`: SIMD algorithm
* (optional) `output_format`: the output format to use - defaults to `PBM`
  * `0`: PBM: Portable BitMap format (black & white output)
  * `1`: PPM: Portable PixMap format (colored output)
  
`cargo run --release -- 400` outputs:

![run_400_png](https://user-images.githubusercontent.com/904614/43190942-72bdb834-8ffa-11e8-9dcf-a9a9632ae907.png)

`cargo run --releae -- 400 400 1 1` outputs:

![run_400_400_1_1_png](https://user-images.githubusercontent.com/904614/43190948-759969a4-8ffa-11e8-81a9-35e5baef3e86.png)

## Performance

```
./benchmark.sh
```

| 800 x 800  | time [ms] <br> Rust | speedup [-] |
|------------|---------------------|-------------|
| `scalar`   | 86.6                | 1.0x        |
| `simd`     | 46.2                | 1.9x        |
| `par_simd` | 21.0                | 4.1x        |
| `ispc`     | 24.6                | 3.5x        |

On my system the `par_simd` algorithm is ~1.2x faster than ISPC. While both
algorithms produce the same output, they are however different. The `par_simd`
implementation writes the formatted output in parallel inside from the main
loop, while the ISPC algorithm computes the mandelbrot set first saving it to
memory, and subsequently loads it from memory again to do the formatting.

[bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/mandelbrot.html#mandelbrot
