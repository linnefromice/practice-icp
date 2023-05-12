dfx stop && dfx start --background --clean

# deploy
dfx deploy snapshotter_v2
dfx deploy calculator_v2
dfx deploy relayer_v2

# initialize snapshotter_v2
dfx canister call snapshotter_v2 setup '(
    "https://opt-mainnet.g.alchemy.com/v2/fA5-W40wt75OemXkr7OjoxT6gbxkBgSI",
    "85149247691df622eaf1a8bd0cafd40bc45154a9",
    opt 300,
)'
# dfx canister call snapshotter_v2 bulk_save_prices '(
#     vec {},
#     null,
#     null
# )'
dfx canister call snapshotter_v2 set_task '(
    opt 30,
    null,
    null
)'

# initialize relayer_v2
## prerequisites: transfer native token to relayer_v2
dfx canister call relayer_v2 setup "(
    \"https://polygon-mumbai.g.alchemy.com/v2/6GLIzI5pL0n4bp4c3jESZTRfXxE5XJ_Z\",
    80001,
    \"E5f0DA5761B82e14E45021246EE657D07a9BBd23\",
    \"$(dfx canister id calculator_v2)\",
    \"$(dfx canister id snapshotter_v2)\",
    18,
    6,
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
#     18,
#     6,
#     18,
#     null
# )"
dfx canister call calculator_v2 debug_calculate_exchange_rates_for_prices "(
    \"$(dfx canister id snapshotter_v2)\",
    18,
    6,
    18,
    null,
    null,
)"
