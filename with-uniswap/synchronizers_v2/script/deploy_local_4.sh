dfx stop && dfx start --background --clean

dfx deploy reader_v2 \
    --argument '(
        "https://eth-mainnet.alchemyapi.io/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy",
        "88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
        opt 300,
    )'
dfx deploy calculator_v2
dfx deploy relayer_v2

dfx canister call reader_v2 bulk_save_prices '(
    vec {
        16836712;
        17035261;
        17035262;
        17162287;
        17162288;
        17169397;
        17169398;
        17176520;
        17176521;
        17183617;
        17183618;
        17190742;
        17190743;
        17197868;
        17197869
    },
    null,
    null
)'

# debug
dfx canister call calculator_v2 debug_last_1day "(null)"
dfx canister call calculator_v2 get_last_day_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, null)"
dfx canister call relayer_v2 debug_call_get_last_day_realized_volatility "(\"$(dfx canister id calculator_v2)\", \"$(dfx canister id reader_v2)\", 6, 18, 10, null)"

dfx canister call calculator_v2 debug_last_4week "(null)"
dfx canister call calculator_v2 get_last_4week_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, null)"
dfx canister call relayer_v2 debug_call_get_last_4week_realized_volatility "(\"$(dfx canister id calculator_v2)\", \"$(dfx canister id reader_v2)\", 6, 18, 10, null)"

# with oracle
## prepares: set parameters
# dfx canister call relayer_v2 debug_sync_state_internal "(100, null, null, null)"

# dfx canister call relayer_v2 debug_call_get_last_day_realized_volatility "(\"$(dfx canister id calculator_v2)\", \"$(dfx canister id reader_v2)\", 6, 18, 10, null)"
# dfx canister call relayer_v2 debug_call_get_last_day_realized_volatility "(\"$(dfx canister id calculator_v2)\", \"$(dfx canister id reader_v2)\", 6, 18, 10, opt 1)"
# dfx canister call relayer_v2 debug_call_get_last_day_realized_volatility "(\"$(dfx canister id calculator_v2)\", \"$(dfx canister id reader_v2)\", 6, 18, 10, opt 2)"
# dfx canister call relayer_v2 debug_call_get_last_day_realized_volatility "(\"$(dfx canister id calculator_v2)\", \"$(dfx canister id reader_v2)\", 6, 18, 10, opt 3)"
# dfx canister call relayer_v2 debug_call_get_last_day_realized_volatility "(\"$(dfx canister id calculator_v2)\", \"$(dfx canister id reader_v2)\", 6, 18, 10, opt 4)"

# dfx canister call relayer_v2 debug_sync_state "(\"$(dfx canister id calculator_v2)\", \"$(dfx canister id reader_v2)\", 6, 18, 10, null, null)"
# dfx canister call relayer_v2 debug_sync_state "(\"$(dfx canister id calculator_v2)\", \"$(dfx canister id reader_v2)\", 6, 18, 10, null, opt 1)"
