#!/usr/bin/env bash
#
# Runs options_pricing benchmarks

set -ex

NUM_OPTIONS=10000000

if [[ ${NORUN} != 1 ]]; then
    hash hyperfine 2>/dev/null || { echo >&2 "hyperfine is not in PATH."; exit 1; }
fi

ALGS=("black_scholes_scalar" "black_scholes_simd" "binomial_put_scalar" "binomial_put_simd")
if echo "$FEATURES" | grep -q "ispc"; then
    hash ispc 2>/dev/null || { echo >&2 "ispc is not in PATH."; exit 1; }
    ALGS+=("black_scholes_ispc" "black_scholes_ispc_tasks" "binomial_put_ispc" "binomial_put_ispc_tasks")
fi

RUSTFLAGS="-C target-cpu=native ${RUSTFLAGS}" \
         cargo build --release --features="${FEATURES}"

if [[ "${NORUN}" == "1" ]]; then
    exit 0
fi

for alg in "${ALGS[@]}"
do
    hyperfine "target/release/options_pricing ${NUM_OPTIONS} ${alg}"
done
