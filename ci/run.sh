#!/bin/sh

set -ex

: ${TARGET?"The TARGET environment variable must be set."}

# Tests are all super fast anyway, and they fault often enough on travis that
# having only one thread increases debuggability to be worth it.
export RUST_TEST_THREADS=1
#export RUST_BACKTRACE=full
#export RUST_TEST_NOCAPTURE=1

export CARGO_SUBCMD=test
if [[ $NORUN == 1 ]]; then
    export CARGO_SUBCMD=build
fi

echo "RUSTFLAGS=${RUSTFLAGS}"
echo "FEATURES=${FEATURES}"
echo "NORUN=${NORUN}"
echo "CARGO_SUBCMD=${CARGO_SUBCMD}"

cargo_test() {
    cmd="cargo $CARGO_SUBCMD --target=$TARGET $1"
    $cmd
}
cargo_test
cargo_test "--release"

# Test different feature sets.
case ${TARGET} in
    x86*)
        RUSTFLAGS="${RUSTFLAGS} -C target-feature=+sse4.2"
        cargo_test "--release"
        RUSTFLAGS="${RUSTFLAGS} -C target-feature=+avx"
        cargo_test "--release"
        RUSTFLAGS="${RUSTFLAGS} -C target-feature=+avx2"
        cargo_test "--release"
        ;;
    armv7*)
        RUSTFLAGS="${RUSTFLAGS} -C target-feature=+neon"
        cargo_test "--release"
        ;;
    aarch64*)
        RUSTFLAGS="${RUSTFLAGS} -C target-feature=+neon"
        cargo_test "--release"
        ;;
    mips64*)
        RUSTFLAGS="${RUSTFLAGS} -C target-feature=+msa"
        cargo_test "--release"
        ;;
    powerpc-*)
        RUSTFLAGS="${RUSTFLAGS} -C target-feature=+altivec"
        cargo_test "--release"
        ;;
    powerpc64-*)
        RUSTFLAGS="${RUSTFLAGS} -C target-feature=+altivec"
        cargo_test "--release"
        RUSTFLAGS="${RUSTFLAGS} -C target-feature=+vsx"
        cargo_test "--release"
        ;;
    *)
        ;;
esac
