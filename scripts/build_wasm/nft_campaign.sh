#!/bin/bash
set -e

cd ../../contracts/campaigns/nft_campaign
cargo build --target wasm32-unknown-unknown --release
cd ../../../
cp target/wasm32-unknown-unknown/release/nft_campaign.wasm wasm
