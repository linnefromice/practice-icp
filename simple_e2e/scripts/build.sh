# hello_motoko
echo "Building hello_motoko"
$(dfx cache show)/moc --idl -o artifacts/hello_motoko.did src/hello_motoko/Main.mo
$(dfx cache show)/moc -v -o artifacts/hello_motoko.wasm src/hello_motoko/Main.mo

# hello_rust
echo "Building hello_rust"
cp -f src/hello_rust/hello_rust.did artifacts/hello_rust.did
cargo build --target wasm32-unknown-unknown --release --package hello_rust
ic-wasm target/wasm32-unknown-unknown/release/hello_rust.wasm -o artifacts/hello_rust.wasm shrink

# counter_motoko
echo "Building counter_motoko"
$(dfx cache show)/moc --idl -o artifacts/counter_motoko.did src/counter_motoko/Main.mo
$(dfx cache show)/moc -v -o artifacts/counter_motoko.wasm src/counter_motoko/Main.mo

# counter_rust
echo "Building counter_rust"
cp -f src/counter_rust/counter_rust.did artifacts/counter_rust.did
cargo build --target wasm32-unknown-unknown --release --package counter_rust
ic-wasm target/wasm32-unknown-unknown/release/counter_rust.wasm -o artifacts/counter_rust.wasm shrink

# http_outcall_erc20
echo "Building http_outcall_erc20"
cp -f src/http_outcall_erc20/http_outcall_erc20.did artifacts/http_outcall_erc20.did
cargo build --target wasm32-unknown-unknown --release --package http_outcall_erc20
ic-wasm target/wasm32-unknown-unknown/release/http_outcall_erc20.wasm -o artifacts/http_outcall_erc20.wasm shrink

# http_outcall_pool
echo "Building http_outcall_pool"
cp -f src/http_outcall_pool/http_outcall_pool.did artifacts/http_outcall_pool.did
cargo build --target wasm32-unknown-unknown --release --package http_outcall_pool
ic-wasm target/wasm32-unknown-unknown/release/http_outcall_pool.wasm -o artifacts/http_outcall_pool.wasm shrink
