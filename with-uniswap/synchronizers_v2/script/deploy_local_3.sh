dfx stop && dfx start --background --clean
dfx deploy reader_v2 \
    --argument '(
        "https://eth-mainnet.alchemyapi.io/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy",
        "88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
        opt 300,
    )'
dfx canister call reader_v2 bulk_save_prices '(
    vec {
        17055793;
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

dfx deploy calculator_v2

dfx canister call reader_v2 get_filtered_price_indexes "(true)"
dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\", null, null)"
dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\", opt 1678924800, opt 1681344000)"
dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\", opt 1683158400, opt 1683244800)"

# last 4 week (3/16-4/13)
dfx canister call reader_v2 get_prices "(opt 1678924800, opt 1681344000)"
dfx canister call calculator_v2 get_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, opt 1678924800, opt 1681344000)"
dfx canister call calculator_v2 get_last_4week_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, null)"

# last 1 day
dfx canister call reader_v2 get_prices "(opt 1683158400, opt 1683244800)"
dfx canister call calculator_v2 get_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, opt 1683158400, opt 1683244800)"
dfx canister call calculator_v2 get_last_day_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, null)"

## debug
dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\", null, null)"
dfx canister call calculator_v2 debug_last_1day "(opt 1)"
dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\", opt 1683072000, opt 1683158400)"
dfx canister call calculator_v2 get_last_day_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, opt 1)"
dfx canister call calculator_v2 debug_last_1day "(opt 2)"
dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\", opt 1682985600, opt 1683072000)"
dfx canister call calculator_v2 get_last_day_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, opt 2)"

dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\", opt 1682985600, opt 1683072000)"

dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\", null, null)"
dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\", opt 1682899200, opt 1683072000)"
