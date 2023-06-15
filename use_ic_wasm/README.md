# use_ic_wasm

```bash
% dfx stop && dfx start --background --clean && dfx canister create --all && dfx build

% ic-wasm .dfx/local/canisters/hello_motoko/hello_motoko.wasm metadata
icp:public candid:service
icp:private candid:args
icp:private motoko:stable-types
icp:private motoko:compiler
% ic-wasm .dfx/local/canisters/hello_rust/hello_rust.wasm metadata 
icp:public candid:service
% ic-wasm target/wasm32-unknown-unknown/release/hello_rust.wasm metadata
% 
```

```bash
% cargo build --target wasm32-unknown-unknown --release -p hello_rust --locked 
% ic-wasm target/wasm32-unknown-unknown/release/hello_rust.wasm metadata
% 
```
