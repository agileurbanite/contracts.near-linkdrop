#!/bin/bash
set -e

cd ../../../user
cargo build --target wasm32-unknown-unknown --release
cp ../target/wasm32-unknown-unknown/release/user.wasm ../wasm
