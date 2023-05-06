dfx stop && dfx start --background --clean

# reader_v2
dfx deploy reader_v2 \
    --argument '(
        "https://eth-mainnet.alchemyapi.io/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy",
        "88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
        opt 300,
    )'

# reader_v2: initialize
dfx canister call reader_v2 bulk_save_prices '(
    vec {
        16836711;
        16836712;
        17035261;
        17035262;
        17176520;
        17176521;
        17183617;
        17183618;
        17190742;
        17190743
    },
    null,
    null
)'
# block numbers in mainnet -> observationBlockTimestamp
# 16836712 -> 2023-03-16T00:00:23.000Z
# 16950603 -> 2023-03-31T23:56:35.000Z
# 16950604 -> 2023-04-01T00:00:23.000Z
# 17035261 -> 2023-04-12T23:58:47.000Z
# 17035262 -> 2023-04-13T00:01:11.000Z
# 17055792 -> 2023-04-15T23:58:59.000Z
# 17055793 -> 2023-04-16T00:00:23.000Z
# 17162287 -> 2023-04-30T23:59:47.000Z
# 17162288 -> 2023-05-01T00:00:35.000Z
# 17169397 -> 2023-05-01T23:59:35.000Z
# 17169398 -> 2023-05-02T00:00:35.000Z
# 17176520 -> 2023-05-02T23:57:59.000Z
# 17176521 -> 2023-05-03T00:02:59.000Z
# 17183617 -> 2023-05-03T23:56:59.000Z
# 17183618 -> 2023-05-04T00:02:23.000Z
# 17190742 -> 2023-05-04T23:59:59.000Z
# 17190743 -> 2023-05-05T00:02:47.000Z
# 17197868 -> 2023-05-05T23:59:47.000Z
# 17197869 -> 2023-05-06T00:02:11.000Z

dfx canister call reader_v2 periodic_save_prices '(
    opt 60,
    null,
    null
)'

dfx canister call reader_v2 get_filtered_price_indexes "(false)"
dfx canister call reader_v2 get_filtered_price_indexes "(true)"

# calculator
dfx deploy calculator_v2

dfx canister call calculator_v2 get_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, null, null)"
dfx canister call calculator_v2 get_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10, opt 1680307200, opt 1682899200)"
dfx canister call calculator_v2 get_last_month_realized_volatility "(\"$(dfx canister id reader_v2)\", 6, 18, 10)"
