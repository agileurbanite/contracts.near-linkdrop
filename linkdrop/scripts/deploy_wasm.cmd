set RUSTFLAGS=-C link-arg=-s
cargo build --release --target wasm32-unknown-unknown
near deploy --accountId linkdrop.testnet --wasmFile ../target/wasm32-unknown-unknown/release/linkdrop.wasm