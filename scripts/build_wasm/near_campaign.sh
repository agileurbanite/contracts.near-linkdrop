#!/bin/bash
set -e

cd ../../contracts/campaigns/near_campaign
cargo build --target wasm32-unknown-unknown --release
cp ../../../target/wasm32-unknown-unknown/release/near_campaign.wasm ../../../wasm
