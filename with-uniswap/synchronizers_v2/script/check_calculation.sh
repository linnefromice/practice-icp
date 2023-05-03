dfx canister call calculator_v2 debug_call_prices "(\"$(dfx canister id reader_v2)\")"
dfx canister call calculator_v2 debug_calculate_exchange_rates_for_prices "(\"$(dfx canister id reader_v2)\", 6, 18, 0)"
dfx canister call calculator_v2 debug_calculate_realized_volatility '(vec {"100"; "200"; "300"; "400"; "500"})'
dfx canister call calculator_v2 debug_calculate_realized_volatility_for_prices "(\"$(dfx canister id reader_v2)\", 6, 18, 0)"
