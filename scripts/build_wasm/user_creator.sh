#!/bin/bash
set -e

cd ../../../contracts/linkdrop
cargo build --target wasm32-unknown-unknown --release
cp ../../target/wasm32-unknown-unknown/release/linkdrop.wasm ../../wasm
