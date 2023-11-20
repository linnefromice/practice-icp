.PHONY: build build build_by_manual

build:
	dfx stop && dfx start --clean --background
	dfx canister create --all
	dfx build --all
	cp .dfx/local/canisters/backend_1/backend_1.wasm artifacts/Backend1.wasm
	cp .dfx/local/canisters/backend_2/backend_2.wasm artifacts/Backend2.wasm
	dfx stop

build_by_manual:
	cargo build --target wasm32-unknown-unknown --release --locked -p backend_1
	cargo build --target wasm32-unknown-unknown --release --locked -p backend_2
	ic-wasm target/wasm32-unknown-unknown/release/backend_1.wasm -o artifacts/Backend1_only_shrink.wasm shrink
	ic-wasm target/wasm32-unknown-unknown/release/backend_2.wasm -o artifacts/Backend2_only_shrink.wasm shrink

launch:
	dfx stop && dfx start --background --clean
	dfx deploy backend_1
	dfx deploy backend_2
	dfx deploy manager --argument "(record { backend_1 = \"$(dfx canister id backend_1)\"; backend_2 = \"$(dfx canister id backend_2)\" })"