#!/bin/bash
set -e

cd ../../contracts/user_creator
cargo build --target wasm32-unknown-unknown --release
cp ../../target/wasm32-unknown-unknown/release/user_creator.wasm ../../wasm
