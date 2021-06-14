set RUSTFLAGS=-C link-arg=-s
cargo build --release --target wasm32-unknown-unknown
cd ../target/wasm32-unknown-unknown/release
copy "user.wasm" "../../../../wasm"