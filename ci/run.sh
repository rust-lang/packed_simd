#!/usr/bin/env bash

set -ex

: ${TARGET?"The TARGET environment variable must be set."}

# Tests are all super fast anyway, and they fault often enough on travis that
# having only one thread increases debuggability to be worth it.
export RUST_TEST_THREADS=1
#export RUST_BACKTRACE=full
#export RUST_TEST_NOCAPTURE=1

# Some appveyor builds run out-of-memory; this attempts to mitigate that:
# https://github.com/rust-lang-nursery/packed_simd/issues/39
# export RUSTFLAGS="${RUSTFLAGS} -C codegen-units=1"
# export CARGO_BUILD_JOBS=1

export CARGO_SUBCMD=test
if [[ "${NORUN}" == "1" ]]; then
    export CARGO_SUBCMD=build
fi

if [[ ${TARGET} == "x86_64-apple-ios" ]] || [[ ${TARGET} == "i386-apple-ios" ]]; then
    export RUSTFLAGS="${RUSTFLAGS} -Clink-arg=-mios-simulator-version-min=7.0"
    rustc ./ci/deploy_and_run_on_ios_simulator.rs -o $HOME/runtest
    export CARGO_TARGET_X86_64_APPLE_IOS_RUNNER=$HOME/runtest
    export CARGO_TARGET_I386_APPLE_IOS_RUNNER=$HOME/runtest
fi

rustc --version
cargo --version
echo "TARGET=${TARGET}"
echo "RUSTFLAGS=${RUSTFLAGS}"
echo "FEATURES=${FEATURES}"
echo "NORUN=${NORUN}"
echo "CARGO_SUBCMD=${CARGO_SUBCMD}"
echo "CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS}"
echo "CARGO_INCREMENTAL=${CARGO_INCREMENTAL}"
echo "RUST_TEST_THREADS=${RUST_TEST_THREADS}"
echo "RUST_BACKTRACE=${RUST_BACKTRACE}"
echo "RUST_TEST_NOCAPTURE=${RUST_TEST_NOCAPTURE}"

cargo_test() {
    cmd="cargo ${CARGO_SUBCMD} --verbose --target=${TARGET} ${1}"
    mkdir target || true
    ${cmd} 2>&1 | tee > target/output
    if [[ ${PIPESTATUS[0]} != 0 ]]; then
        cat target/output
        return 1
    fi
}

cargo_test_impl() {
    ORIGINAL_RUSTFLAGS=${RUSTFLAGS}
    RUSTFLAGS="${ORIGINAL_RUSTFLAGS} --cfg test_v16  --cfg test_v32" cargo_test ${1}
    RUSTFLAGS="${ORIGINAL_RUSTFLAGS} --cfg test_v64  --cfg test_v128" cargo_test ${1}
    RUSTFLAGS="${ORIGINAL_RUSTFLAGS} --cfg test_v256 --cfg test_v512" cargo_test ${1}
    RUSTFLAGS=${ORIGINAL_RUSTFLAGS}
}

cargo_test_impl
cargo_test_impl "--release --features=into_bits"
cargo_test_impl "--release --features=into_bits,coresimd"

# Examples - the source directory is read-only.
# Need to copy them to the target directory for the Cargo.lock to be
# properly written.
mkdir target || true

# FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/55
# All examples fail to build for `armv7-apple-ios`.
if [[ ${TARGET} == "armv7-apple-ios" ]]; then
    exit 0
fi

cp -r examples/nbody target/nbody
cargo_test "--manifest-path=target/nbody/Cargo.toml"
cargo_test "--release --manifest-path=target/nbody/Cargo.toml"

# FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/56
if [[ ${TARGET} != "i586-unknown-linux-gnu" ]]; then
    cp -r examples/mandelbrot target/mandelbrot
    cargo_test "--manifest-path=target/mandelbrot/Cargo.toml"
    cargo_test "--release --manifest-path=target/mandelbrot/Cargo.toml"
fi

cp -r examples/spectral_norm target/spectral_norm
cargo_test "--manifest-path=target/spectral_norm/Cargo.toml"
cargo_test "--release --manifest-path=target/spectral_norm/Cargo.toml"

cp -r examples/fannkuch_redux target/fannkuch_redux
cargo_test "--manifest-path=target/fannkuch_redux/Cargo.toml"
cargo_test "--release --manifest-path=target/fannkuch_redux/Cargo.toml"

cp -r examples/aobench target/aobench
cargo_test "--manifest-path=target/aobench/Cargo.toml"
cargo_test "--release --manifest-path=target/aobench/Cargo.toml --no-default-features"
cargo_test "--release --manifest-path=target/aobench/Cargo.toml --features=256bit"
