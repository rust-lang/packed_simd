#!/bin/sh

set -ex

rm -rf target/doc
mkdir -p target/doc

cargo doc --features=into_bits

# If we're on travis, not a PR, and on the right branch, publish!
if [ "$TRAVIS_PULL_REQUEST" = "false" ] && [ "$TRAVIS_BRANCH" = "master" ]; then
  pip install ghp_import --install-option="--prefix=$HOME/.local"
  $HOME/.local/bin/ghp-import -n target/doc
  git push -qf https://${GH_PAGES}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
fi
