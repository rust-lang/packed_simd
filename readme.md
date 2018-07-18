# `Simd<[T; N]>` - Implementation of [RFC2366: `std::simd`](https://github.com/rust-lang/rfcs/pull/2366)

[![Travis-CI Status]][travis] [![Appveyor Status]][appveyor] [![Latest Version]][crates.io] [![docs]][docs.rs]

> This aims to be a 100% conforming implementation of the RFC2366 for stabilization.

# Platform support

The following table describes the supported platforms: `build` means that the
library compiles correctly for the target using an appropriate cross-compilation
toolchain, and `run` means that the full testsuite passes on the target in CI.

| Linux targets:                    | build     | run     |
|-----------------------------------|-----------|---------|
| `i586-unknown-linux-gnu`          | ✓         | ✓       |
| `i686-unknown-linux-gnu`          | ✓         | ✓       |
| `x86_64-unknown-linux-gnu`        | ✓         | ✓       |
| `arm-unknown-linux-gnueabihf`     | ✓         | ✓       |
| `armv7-unknown-linux-gnueabihf`   | ✓         | ✓       |
| `armv7-unknown-linux-gnueabi`     | ✓         | ✓       |
| `aarch64-unknown-linux-gnu`       | ✓         | ✓       |
| `mips-unknown-linux-gnu`          | ✓         | ✓       |
| `mipsel-unknown-linux-musl`       | ✓         | ✓       |
| `mips64-unknown-linux-gnuabi64`   | ✓         | ✓       |
| `mips64el-unknown-linux-gnuabi64` | ✓         | ✓       |
| `powerpc-unknown-linux-gnu`       |           |         |
| `powerpc64-unknown-linux-gnu`     |           |         |
| `powerpc64le-unknown-linux-gnu`   |           |         |
| `s390x-unknown-linux-gnu`         | ✓         | ✓*      |
| `sparc64-unknown-linux-gnu`       | ✓         | ✓*      |
| **MacOSX targets:**               | **build** | **run** |
| `x86_64-apple-darwin`             | ✓         | ✓       |
| `i686-apple-darwin`               | ✓         | ✓       |
| **Windows targets:**              | **build** | **run** |
| `x86_64-pc-windows-msvc`          | ✓         | ✓       |
| `i686-pc-windows-msvc`            | ✓         | ✓       |
| `x86_64-pc-windows-gnu`           | ✓         | ✓       |
| `i686-pc-windows-gnu`             | ✓         | ✓       |
| **Android targets:**              | build     | run     |
| `x86_64-linux-android`            | ✓         | ✓       |
| `arm-linux-androideabi`           | ✓         | ✓       |
| `aarch64-linux-android`           | ✓         | ✓       |
| **iOS targets:**                  | build     | run     |
| `i386-apple-ios`                  | ✓         |         |
| `x86_64-apple-ios`                | ✓         |         |
| `armv7-apple-ios`                 | ✓         | **      |
| `aarch64-apple-ios`               | ✓         | **      |
| **WASM targets:**                 | build     | run     |
| `wasm32-unknown-unknown`          | ✓         | **      |

[*] most of the test suite passes correctly on these platform but
there are correctness bugs open in the issue tracker.

[**] it is currently not easily possible to run these platforms on CI.


# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `ppv` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[travis]: https://travis-ci.org/gnzlbg/ppv
[Travis-CI Status]: https://travis-ci.org/gnzlbg/ppv.svg?branch=master
[appveyor]: https://ci.appveyor.com/project/gnzlbg/ppv/branch/master
[Appveyor Status]: https://ci.appveyor.com/api/projects/status/lobb2qte2q5gbxbo?svg=true
[Latest Version]: https://img.shields.io/crates/v/ppv.svg
[crates.io]: https://crates.io/crates/ppv
[docs]: https://docs.rs/ppv/badge.svg
[docs.rs]: https://docs.rs/ppv/
