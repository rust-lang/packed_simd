#!/usr/bin/env bash

# Cleans all target artifacts

set -ex

cargo_clean() {
    cargo clean
}

# Check src/
cargo_clean

# Check examples/
for dir in examples/*/
do
    dir=${dir%*/}
    cd ${dir%*/}
    cargo_clean
    cd -
done

cd verify/verify
cargo_clean
cd -
