# simple_e2e

```bash
./scripts/build.sh
dfx generate

dfx stop && dfx start --clean --background && dfx canister create --all && dfx build && dfx canister install --all
# confirmation
dfx deploy

dfx canister call counter_motoko label '()'
dfx canister call counter_rust label '()'
dfx canister call counter_motoko get '()'
dfx canister call counter_rust get '()'

cd tests && yarn test --test-timeout=20000
```
