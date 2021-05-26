set RUSTFLAGS=-C link-arg=-s
cargo build --release --target wasm32-unknown-unknown
near dev-deploy ../target/wasm32-unknown-unknown/release/near_campaign.wasm