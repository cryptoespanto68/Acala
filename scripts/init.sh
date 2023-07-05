#!/usr/bin/env bash

set -e

echo "*** Initializing WASM build environment"

rustup default nightly-2023-04-15

rustup target add wasm32-unknown-unknown --toolchain nightly-2023-04-15
