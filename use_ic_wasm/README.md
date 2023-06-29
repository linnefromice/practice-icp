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

# Add arbitrary metadata
% ic-wasm .dfx/local/canisters/hello_motoko/hello_motoko.wasm -o .dfx/local/canisters/hello_motoko/hello_motoko.wasm metadata platform:label -d "hello_motoko" -v public
% ic-wasm .dfx/local/canisters/hello_motoko/hello_motoko.wasm -o .dfx/local/canisters/hello_motoko/hello_motoko.wasm metadata platform:component_type -d "motoko" -v public
% ic-wasm .dfx/local/canisters/hello_motoko/hello_motoko.wasm -o .dfx/local/canisters/hello_motoko/hello_motoko.wasm metadata platform:description -d "Hello! Motoko." -v public
% ic-wasm .dfx/local/canisters/hello_motoko/hello_motoko.wasm metadata
icp:public candid:service
icp:private candid:args
icp:private motoko:stable-types
icp:private motoko:compiler
icp:public platform:label
icp:public platform:component_type
icp:public platform:description
% ic-wasm .dfx/local/canisters/hello_rust/hello_rust.wasm -o .dfx/local/canisters/hello_rust/hello_rust.wasm metadata platform:label -d "hello_rust" -v public
% ic-wasm .dfx/local/canisters/hello_rust/hello_rust.wasm -o .dfx/local/canisters/hello_rust/hello_rust.wasm metadata platform:component_type -d "rust" -v public
% ic-wasm .dfx/local/canisters/hello_rust/hello_rust.wasm -o .dfx/local/canisters/hello_rust/hello_rust.wasm metadata platform:description -d "Hello! Rust." -v public
% ic-wasm .dfx/local/canisters/hello_rust/hello_rust.wasm metadata
icp:public candid:service
icp:public platform:label
icp:public platform:component_type
icp:public platform:description
```

## Appendix

```bash
% cargo build --target wasm32-unknown-unknown --release -p hello_rust --locked 
% ic-wasm target/wasm32-unknown-unknown/release/hello_rust.wasm metadata
% 
```
