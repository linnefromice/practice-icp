# use_stable_structure

```bash
cargo fmt
cargo clippy -- -D clippy::all

cargo make did \
  && dfx stop \
  && dfx start --clean --background \
  && dfx canister create --all \
  && dfx build \
  && dfx canister install --all \
  && dfx deploy
```
