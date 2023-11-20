# ws_install_code

```bash
dfx stop && dfx start --background --clean && dfx deploy

dfx canister call backend_1 get_player
dfx canister call backend_1 greet '("backend_1")'
dfx canister call backend_2 get_player
dfx canister call backend_2 greet '("backend_2")'
```
