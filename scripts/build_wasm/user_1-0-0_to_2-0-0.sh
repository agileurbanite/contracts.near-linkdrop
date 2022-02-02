#!/bin/bash
set -e

cd ../../migrations/user_1-0-0_to_2-0-0
cargo build --target wasm32-unknown-unknown --release
cd ../../
# somehow it converts user_1-0-0_to_2-0-0.wasm into user_1_0_0_to_2_0_0.wasm
cp target/wasm32-unknown-unknown/release/user_1_0_0_to_2_0_0.wasm wasm
cd wasm
# rename it back
mv user_1_0_0_to_2_0_0.wasm user_1-0-0_to_2-0-0.wasm