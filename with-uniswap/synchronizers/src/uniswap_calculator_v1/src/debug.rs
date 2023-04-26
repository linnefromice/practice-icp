use candid::Principal;
use ic_cdk::{api::call, query, update};
use ic_web3::types::U256;

use crate::{
    call_get_prices_in_reader, get_prices_from_all_registered, token0_decimals, token1_decimals,
    utils::calculate_exchange_rate, CandidPrice, READER_CANISTERS,
};

#[update]
async fn debug_rpcs() -> Vec<(String, String)> {
    let canisters = READER_CANISTERS.with(|value| value.borrow().clone());
    let mut results = Vec::<(String, String)>::new();
    for canister in canisters {
        let result = call::call::<_, (String,)>(canister, "debug_get_rpc_url", ()).await;
        results.push((canister.to_string(), result.unwrap().0));
    }
    results
}
#[query]
fn debug_tokens_decimals() -> (u8, u8) {
    (token0_decimals(), token1_decimals())
}

#[update]
async fn debug_get_prices_from_all_registered(
    from: Option<u32>,
    to: Option<u32>,
) -> Vec<Vec<CandidPrice>> {
    get_prices_from_all_registered(from, to).await
}
#[update]
async fn debug_call_get_prices_in_reader(
    canister_id: Principal,
    from: Option<u32>,
    to: Option<u32>,
) -> Vec<CandidPrice> {
    call_get_prices_in_reader(canister_id, from, to).await
}
#[query]
fn debug_calculate_exchange_rate(sqrt_price_x96: String, precision: u8) -> String {
    let result = calculate_exchange_rate(
        U256::from_dec_str(&sqrt_price_x96).unwrap(),
        token0_decimals(),
        token1_decimals(),
        precision,
    );
    result.to_string()
}
