.PHONY: build launch

build:
	cargo build --target wasm32-unknown-unknown --release --locked -p backend_1
	cargo build --target wasm32-unknown-unknown --release --locked -p backend_2
	ic-wasm target/wasm32-unknown-unknown/release/backend_1.wasm -o artifacts/Backend1.wasm shrink
	ic-wasm target/wasm32-unknown-unknown/release/backend_2.wasm -o artifacts/Backend2.wasm shrink

launch:
	dfx stop && dfx start --background --clean && dfx deploy