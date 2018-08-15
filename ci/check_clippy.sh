#!/usr/bin/env bash

# Uses clippy to check the idioms of the library

set -ex

cargo_clippy() {
    cargo clippy --all -- -D clippy-pedantic
}

# Check src/
cargo_clippy

# Check examples/
for dir in examples/*/
do
    dir=${dir%*/}
    cd ${dir%*/}
    cargo_clippy
    cd -
done

cd verify
cargo_clippy
cd -
