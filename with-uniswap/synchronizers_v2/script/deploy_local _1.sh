dfx stop && dfx start --background --clean

# reader_v2
dfx deploy reader_v2 \
    --argument '(
        "https://eth-mainnet.alchemyapi.io/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy",
        "88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
        opt 60,
    )'

# reader_v2: initialize
dfx canister call reader_v2 bulk_save_prices '(
    vec {
        17179465;
        17179759;
        17180054;
        17180352;
    },
    null,
    null
)'

dfx canister call reader_v2 periodic_save_prices '(
    opt 10,
    null,
    null
)'

dfx canister call reader_v2 get_price_indexes "(false)"
dfx canister call reader_v2 get_price_indexes "(true)"
dfx canister call reader_v2 get_filtered_price_indexes "(false)"
dfx canister call reader_v2 get_filtered_price_indexes "(true)"

dfx canister call reader_v2 get_prices "(null, null)"
dfx canister call reader_v2 get_prices "(opt 1683118740, null)"
dfx canister call reader_v2 get_prices "(null, opt 1683118740)"
dfx canister call reader_v2 get_prices "(opt 1_683_111_540, opt 1683118740)"

# calculator
dfx deploy calculator_v2

dfx canister call calculator_v2 debug_calculate_realized_volatility_by_setted_prices '(vec {"100"; "200"; "300"; "400"; "500"})'
dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\", null, null)"
dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\", null, opt 1683118740)"
dfx canister call calculator_v2 debug_calculate_exchange_rates_for_prices "(\"$(dfx canister id reader_v2)\", 6, 18, 10, null, null)"
dfx canister call calculator_v2 debug_calculate_exchange_rates_for_prices "(\"$(dfx canister id reader_v2)\", 6, 18, 10, null, opt 1683118740)"
dfx canister call calculator_v2 get_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, null, null)"
dfx canister call calculator_v2 get_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, null, opt 1683118740)"
