#!/bin/bash
set -e

cd ../../../linkdrop
cargo build --target wasm32-unknown-unknown --release
cp ../target/wasm32-unknown-unknown/release/linkdrop.wasm ../wasm
