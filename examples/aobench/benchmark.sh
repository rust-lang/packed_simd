#!/usr/bin/env bash
#
# Runs aobench benchmarks

set -ex

export WIDTH=800
export HEIGHT=600

ALGS=("scalar" "scalar_par" "vector" "vector_par" "tiled" "tiled_par")
if echo "$FEATURES" | grep -q "ispc"; then
    ALGS+=("ispc" "ispc_tasks")
fi

echo "Benchmark 256-bit wide vectors"
RUSTFLAGS="-C target-cpu=native ${RUSTFLAGS}" \
         cargo build --release --no-default-features \
         --features="${FEATURES},256bit"
if [[ "${NORUN}" == "1" ]]; then
    exit 0
fi

for alg in "${ALGS[@]}"
do
    hyperfine "target/release/aobench ${WIDTH} ${HEIGHT} --algo ${alg}"
done

echo "Benchmark 128-bit wide vectors"
RUSTFLAGS="-C target-cpu=native ${RUSTFLAGS}" \
         cargo build --release --no-default-features \
         --features="${FEATURES}"
for alg in "${ALGS[@]}"
do
    hyperfine "target/release/aobench ${WIDTH} ${HEIGHT} --algo ${alg}"
done
