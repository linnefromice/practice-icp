dfx stop && dfx start --background --clean
dfx deploy reader_mainnet \
    --argument '("https://eth-mainnet.g.alchemy.com/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy","88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640")'
dfx deploy reader_polygon \
    --argument '("https://polygon-mainnet.g.alchemy.com/v2/sLp6VfuskMEwx8Wx0DvaRkI8qCoVYF8f","45dda9cb7c25131df268515131f647d726f50608")'
dfx deploy calculator \
    --argument '(6, 18)'
dfx deploy synchronizer \
    --argument "(\"https://polygon-mumbai.g.alchemy.com/v2/6GLIzI5pL0n4bp4c3jESZTRfXxE5XJ_Z\", 80001, \"5d666338118763ca0cF6719F479491B76bc88131\", principal \"$(dfx canister id calculator)\")"
dfx canister call reader_mainnet periodic_save_prices "(null,null,null)"
dfx canister call reader_polygon periodic_save_prices "(null,null,null)"
dfx canister call calculator register_reader "(principal \"$(dfx canister id reader_mainnet)\")"
dfx canister call calculator register_reader "(principal \"$(dfx canister id reader_polygon)\")"

