# ws_canister_status

```bash
dfx start --background --clean
dfx deploy
dfx canister update-settings --add-controller $(dfx canister id backend_1) backend_2
dfx canister update-settings --add-controller $(dfx canister id backend_1) backend_3

dfx canister call backend_1 call_status_self
dfx canister call backend_1 call_status "$(dfx canister id backend_2)"
dfx canister call backend_1 call_status "$(dfx canister id backend_3)"
```