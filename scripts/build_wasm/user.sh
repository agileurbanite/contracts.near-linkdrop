#!/bin/bash
set -e

cd ../../contracts/user
cargo build --target wasm32-unknown-unknown --release
cp ../../target/wasm32-unknown-unknown/release/user.wasm ../../wasm
