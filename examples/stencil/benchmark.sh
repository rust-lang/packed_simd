#!/usr/bin/env bash
#
# Runs aobench benchmarks

set -ex

algs=("0" "1" "2")
if echo "$FEATURES" | grep -q "ispc"; then
    algs+=( "3" "4" )
fi

RUSTFLAGS="-C target-cpu=native ${RUSTFLAGS}" \
         cargo build --release --no-default-features \
         --features="${FEATURES}"

if [[ "${NORUN}" == "1" ]]; then
    exit 0
fi

for alg in "${algs[@]}"
do
    hyperfine --show-output "target/release/stencil ${alg}"
done
