dfx stop && dfx start --background --clean

# deploy
dfx deploy snapshotter_v2
dfx deploy calculator_v2
dfx deploy relayer_v2

# initialize snapshotter_v2
dfx canister call snapshotter_v2 setup '(
    "https://eth-mainnet.alchemyapi.io/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy",
    "88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    opt 300,
)'
dfx canister call snapshotter_v2 bulk_save_prices '(
    vec {
        17197868;
        17197869;
        17202000;
        17205005;
        17205006
    },
    null,
    null
)'
# dfx canister call reader_v2 set_task '(
#     opt 60,
#     null,
#     null
# )'

# initialize relayer_v2
## prerequisites: transfer native token to relayer_v2
dfx canister call relayer_v2 setup "(
    \"https://polygon-mumbai.g.alchemy.com/v2/6GLIzI5pL0n4bp4c3jESZTRfXxE5XJ_Z\",
    80001,
    \"E5f0DA5761B82e14E45021246EE657D07a9BBd23\",
    \"$(dfx canister id calculator_v2)\",
    \"$(dfx canister id snapshotter_v2)\",
    6,
    18,
    10,
    null
)"
# dfx canister call relayer_v2 set_task "(
#     opt 60,
#     opt 10
# )"
## for debug
# dfx canister call relayer_v2 debug_call_get_last_day_realized_volatility "(
#     \"$(dfx canister id calculator_v2)\",
#     \"$(dfx canister id snapshotter_v2)\",
#     6,
#     18,
#     10,
#     null
# )"