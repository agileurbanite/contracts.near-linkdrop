#!/bin/bash
set -e

cd ../../../contracts/nft-campaign
cargo build --target wasm32-unknown-unknown --release
cp ../../target/wasm32-unknown-unknown/release/nft_campaign.wasm ../../wasm
