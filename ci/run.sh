#!/usr/bin/env bash

set -ex

: ${TARGET?"The TARGET environment variable must be set."}

# Tests are all super fast anyway, and they fault often enough on travis that
# having only one thread increases debuggability to be worth it.
export RUST_TEST_THREADS=1
#export RUST_BACKTRACE=full
#export RUST_TEST_NOCAPTURE=1

export CARGO_SUBCMD=test
if [[ "${NORUN}" == "1" ]]; then
    export CARGO_SUBCMD=build
fi

echo "TARGET=${TARGET}"
rustc --version
echo "RUSTFLAGS=${RUSTFLAGS}"
echo "FEATURES=${FEATURES}"
echo "NORUN=${NORUN}"
echo "CARGO_SUBCMD=${CARGO_SUBCMD}"
echo "RUST_TEST_THREADS=${RUST_TEST_THREADS}"
echo "RUST_BACKTRACE=${RUST_BACKTRACE}"
echo "RUST_TEST_NOCAPTURE=${RUST_TEST_NOCAPTURE}"

cargo_test() {
    cmd="cargo ${CARGO_SUBCMD} --target=${TARGET} ${1}"
    mkdir target || true
    $cmd |& tee > target/output
    if [[ ${PIPESTATUS[0]} != 0 ]]; then
        cat target/output
        return 1
    fi
}

case ${TARGET} in
    x86_64-apple-ios)
        # Note: this case must go before the catch-all "x86*" case below
        export RUSTFLAGS=-Clink-arg=-mios-simulator-version-min=7.0
        rustc ./ci/deploy_and_run_on_ios_simulator.rs -o $HOME/runtest
        export CARGO_TARGET_X86_64_APPLE_IOS_RUNNER=$HOME/runtest

        cargo_test
        cargo_test "--release" "--features=into_bits"
        ;;
    i386-apple-ios)
        export RUSTFLAGS=-Clink-arg=-mios-simulator-version-min=7.0
        rustc ./ci/deploy_and_run_on_ios_simulator.rs -o $HOME/runtest
        export CARGO_TARGET_I386_APPLE_IOS_RUNNER=$HOME/runtest

        cargo_test
        cargo_test "--release" "--features=into_bits"
        ;;
    x86*)
        if [[ ${TARGET} == *"ios"* ]]; then
            echo "ERROR: ${TARGET} must run in the iOS simulator"
            exit 1
        fi

        cargo_test
        cargo_test "--release" "--features=into_bits"

        ORIGINAL_RUSFTFLAGS=${RUSTFLAGS}

        export RUSTFLAGS="${ORIGINAL_RUSTFLAGS} -C target-feature=+sse4.2"
        cargo_test "--release" "--features=into_bits"
        export RUSTFLAGS="${ORIGINAL_RUSTFLAGS} -C target-feature=+avx2"
        cargo_test "--release" "--features=into_bits"

        export RUSTFLAGS=${ORIGINAL_RUSFTFLAGS}
        ;;
    armv7*)
        cargo_test
        cargo_test "--release" "--features=into_bits"

        export RUSTFLAGS="${RUSTFLAGS} -C target-feature=+neon"
        cargo_test "--release" "--features=into_bits"
        ;;
    arm*)
        cargo_test
        cargo_test "--release" "--features=into_bits"

        export RUSTFLAGS="${RUSTFLAGS} -C target-feature=+v7,+neon"
        cargo_test "--release" "--features=into_bits"
        ;;
    aarch64*)
        cargo_test
        cargo_test "--release" "--features=into_bits"

        export RUSTFLAGS="${RUSTFLAGS} -C target-feature=+neon"
        cargo_test "--release" "--features=into_bits"
        ;;
    mips64*)
        cargo_test
        cargo_test "--release" "--features=into_bits"

        # FIXME: this doesn't compile succesfully
        # https://github.com/gnzlbg/packed_simd/issues/18
        #
        export RUSTFLAGS="${RUSTFLAGS} -C target-feature=+msa -C target-cpu=mips64r6"
        cargo_test "--release" "--features=into_bits"
        ;;
    powerpc-*)
        cargo_test
        cargo_test "--release" "--features=into_bits"

        export RUSTFLAGS="${RUSTFLAGS} -C target-feature=+altivec"
        cargo_test "--release" "--features=into_bits"
        ;;
    powerpc64-*)
        cargo_test
        cargo_test "--release" "--features=into_bits"

        ORIGINAL_RUSFTFLAGS=${RUSTFLAGS}

        export RUSTFLAGS="${ORIGINAL_RUSTFLAGS} -C target-feature=+altivec"
        cargo_test "--release" "--features=into_bits"
        export RUSTFLAGS="${ORIGINAL_RUSTFLAGS} -C target-feature=+vsx"
        cargo_test "--release" "--features=into_bits"

        export RUSTFLAGS=${ORIGINAL_RUSFTFLAGS}
        ;;
    *)
        cargo_test
        cargo_test "--release" "--features=into_bits"

        ;;
esac

# Examples
cargo test --release --manifest-path=examples/nbody/Cargo.toml
