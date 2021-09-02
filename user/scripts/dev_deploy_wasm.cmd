cargo build --release --target wasm32-unknown-unknown
near dev-deploy ../target/wasm32-unknown-unknown/release/user.wasm