#!/bin/bash
set -e

cd ../../contracts/campaign_creators/nft_campaign_creator
cargo build --target wasm32-unknown-unknown --release
cp ../../../target/wasm32-unknown-unknown/release/nft_campaign_creator.wasm ../../../wasm
