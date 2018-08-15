#!/usr/bin/env bash

# Uses rustfmt to check the formatting of the library

set -ex

cargo_fmt() {
    cargo fmt --all -- --check
}

# Check src/
cargo_fmt

# Check examples/
for dir in examples/*/
do
    dir=${dir%*/}
    cd ${dir%*/}
    cargo_fmt
    cd -
done

cd verify
cargo_fmt
cd -
