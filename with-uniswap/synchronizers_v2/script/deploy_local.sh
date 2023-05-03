dfx stop && dfx start --background --clean
dfx deploy reader_v2 \
    --argument '(
        "https://eth-mainnet.alchemyapi.io/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy",
        "88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
        opt 60,
    )'
# dfx deploy calculator_v2

dfx canister call reader_v2 bulk_save_prices '(
    vec {
        17179759;
        17180054;
        17180352
    },
    null,
    null
)'

dfx canister call reader_v2 periodic_save_prices '(
    opt 10,
    null,
    null
)'

dfx canister call reader_v2 get_prices "(null, null)"
dfx canister call reader_v2 get_price_indexes "(false)"
dfx canister call reader_v2 get_price_indexes "(true)"
dfx canister call reader_v2 get_filtered_price_indexes "(false)"
dfx canister call reader_v2 get_filtered_price_indexes "(true)"
