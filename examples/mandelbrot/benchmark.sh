#!/usr/bin/env bash
#
# Runs mandelbrot benchmarks

set -ex

WIDTH=800
HEIGHT=800

RUSTFLAGS="-C target-cpu=native ${RUSTFLAGS}" \
         cargo build --release --features="${FEATURES}"

if [[ "${NORUN}" == "1" ]]; then
    exit 0
fi

hyperfine "target/release/mandelbrot ${WIDTH} ${HEIGHT} 0"
hyperfine "target/release/mandelbrot ${WIDTH} ${HEIGHT} 1"
hyperfine "target/release/mandelbrot ${WIDTH} ${HEIGHT} 2"

if echo "$FEATURES" | grep -q "ispc"; then
    hyperfine "target/release/mandelbrot ${WIDTH} ${HEIGHT} 3"
fi
