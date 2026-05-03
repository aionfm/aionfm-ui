#!/usr/bin/env sh
set -eu

cargo fmt -- --check
CARGO_INCREMENTAL=0 cargo check
CARGO_INCREMENTAL=0 cargo test
