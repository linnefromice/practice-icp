use candid::{CandidType, Principal};
use common::{types::CandidPrice, utils::round_timestamp};
use ic_cdk::{query, update};
use ic_web3::types::U256;

use crate::{
    calculate_exchange_rates_for_prices, calculate_last_xxx_realized_volatility, call_prices,
    utils::{calculate_realized_volatility, current_time_sec},
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

#[update]
async fn debug_get_last_5minites_realized_volatility(
    canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    back_terms: Option<u8>,
) -> Result<String, String> {
    calculate_last_xxx_realized_volatility(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        5 * 60,
        back_terms,
    )
    .await
}

#[update]
async fn debug_get_last_10minites_realized_volatility(
    canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    back_terms: Option<u8>,
) -> Result<String, String> {
    calculate_last_xxx_realized_volatility(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        10 * 60,
        back_terms,
    )
    .await
}

#[update]
async fn debug_get_last_30minites_realized_volatility(
    canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    back_terms: Option<u8>,
) -> Result<String, String> {
    calculate_last_xxx_realized_volatility(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        30 * 60,
        back_terms,
    )
    .await
}

#[derive(CandidType)]
pub struct CandidTimestamps {
    pub from: u32,
    pub to: u32,
}
#[query]
fn debug_last_4week(back_terms: Option<u8>) -> CandidTimestamps {
    debug_last_xxx(4 * 7 * 24 * 60 * 60, back_terms)
}
#[query]
fn debug_last_1day(back_terms: Option<u8>) -> CandidTimestamps {
    debug_last_xxx(24 * 60 * 60, back_terms)
}
#[query]
fn debug_last_30min(back_terms: Option<u8>) -> CandidTimestamps {
    debug_last_xxx(30 * 60, back_terms)
}
#[query]
fn debug_last_10min(back_terms: Option<u8>) -> CandidTimestamps {
    debug_last_xxx(10 * 60, back_terms)
}
#[query]
fn debug_last_5min(back_terms: Option<u8>) -> CandidTimestamps {
    debug_last_xxx(5 * 60, back_terms)
}
fn debug_last_xxx(time_unit: u32, back_terms: Option<u8>) -> CandidTimestamps {
    let mut rounded_current_time = round_timestamp(current_time_sec(), time_unit);
    if let Some(value) = back_terms {
        rounded_current_time -= value as u32 * time_unit;
    }
    CandidTimestamps {
        from: rounded_current_time - time_unit,
        to: rounded_current_time,
    }
}
