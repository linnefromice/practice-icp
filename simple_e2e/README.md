# simple_e2e

```bash
./scripts/build.sh
dfx generate

dfx stop && dfx start --clean --background && dfx canister create --all && dfx build && dfx canister install --all
# confirmation
dfx deploy

dfx canister call counter_motoko name '()'
dfx canister call counter_motoko get '()'
dfx canister call counter_rust name '()'
dfx canister call counter_rust get '()'
dfx canister call http_outcall_erc20 name '()'
dfx canister call http_outcall_erc20 symbol '()'
dfx canister call http_outcall_erc20 total_supply '()'
dfx canister call http_outcall_pool token0 '()'
dfx canister call http_outcall_pool token1 '()'
dfx canister call http_outcall_pool fee '()'
dfx canister call http_outcall_pool slot0 '()'

cd tests && yarn test --test-timeout=30000
```
