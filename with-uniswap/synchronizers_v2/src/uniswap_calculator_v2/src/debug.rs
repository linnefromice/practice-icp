use candid::Principal;
use ic_cdk::update;
use ic_web3::types::U256;

use crate::{
    calculate_exchange_rates_for_prices, call_prices, types::CandidPrice,
    utils::calculate_realized_volatility,
};

#[update]
async fn debug_call_prices(
    canister_id: String,
    from: Option<u32>,
    to: Option<u32>,
) -> Result<Vec<CandidPrice>, String> {
    let canister_id = Principal::from_text(canister_id).unwrap();
    call_prices(canister_id, from, to).await
}

#[update]
async fn debug_calculate_exchange_rates_for_prices(
    canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    from: Option<u32>,
    to: Option<u32>,
) -> Result<Vec<String>, String> {
    let canister_id = Principal::from_text(canister_id).unwrap();
    calculate_exchange_rates_for_prices(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        from,
        to,
    )
    .await
    .map(|v| v.iter().map(|u| u.to_string()).collect())
}

#[update]
async fn debug_calculate_realized_volatility_by_setted_prices(
    prices: Vec<String>,
) -> Result<String, String> {
    let prices: Vec<U256> = prices
        .iter()
        .map(|s| U256::from_dec_str(s).unwrap())
        .collect();
    let rv = calculate_realized_volatility(&prices);
    Ok(rv.to_string())
}
