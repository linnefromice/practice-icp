ENV_SRC_RPC_URL="https://opt-mainnet.g.alchemy.com/v2/fA5-W40wt75OemXkr7OjoxT6gbxkBgSI"
ENV_SRC_POOL_ADDR="85149247691df622eaf1a8bd0cafd40bc45154a9"
ENV_DST_RPC_URL="https://polygon-mumbai.g.alchemy.com/v2/6GLIzI5pL0n4bp4c3jESZTRfXxE5XJ_Z"
ENV_DST_CHAIN_ID=80001
ENV_DST_ORACLE_ADDR="E5f0DA5761B82e14E45021246EE657D07a9BBd23"

# initialize snapshotter
dfx canister call snapshotter setup "(
    \"$ENV_SRC_RPC_URL\",
    \"$ENV_SRC_POOL_ADDR\",
    opt 300,
)"

# initialize relayer
## prerequisites: transfer native token to relayer
dfx canister call relayer setup "(
    \"$ENV_DST_RPC_URL\",
    $ENV_DST_CHAIN_ID,
    \"$ENV_DST_ORACLE_ADDR\",
    \"$(dfx canister id calculator)\",
    \"$(dfx canister id snapshotter)\",
    18,
    6,
    10,
    86400,
    null
)"
