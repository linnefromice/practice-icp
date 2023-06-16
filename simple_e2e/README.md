# simple_e2e

```bash
dfx stop && dfx start --clean --background && dfx canister create --all && dfx build && dfx canister install --all
# confirmation
dfx deploy

dfx canister call counter_motoko label '()'
dfx canister call counter_rust label '()'
dfx canister call counter_motoko get '()'
dfx canister call counter_rust get '()'

cd tests && yarn test --test-timeout=20000
```
