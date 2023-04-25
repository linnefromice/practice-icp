use crate::{
    call_observation, call_slot0,
    constants::DEFAULT_POOL_ADDR,
    last_price, price, prices_length, round_timestamp,
    types::{CandidObservation, CandidPrice, CandidSlot0},
    PRICE_INDEXES,
};
use ic_cdk_macros::{query, update};

#[update]
async fn debug_fetch_slot0(
    pool_address: String,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidSlot0, String> {
    let result = call_slot0(pool_address, max_resp, cycles).await;
    result.map(|v| v.to_candid())
}

#[update]
async fn debug_fetch_observation(
    pool_address: String,
    observation_idx: u16,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidObservation, String> {
    let result = call_observation(pool_address, observation_idx, max_resp, cycles).await;
    result.map(|v| v.to_candid())
}

#[update]
async fn debug_fetch_price(
    pool_address: Option<String>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidPrice, String> {
    let pool_address = if pool_address.is_some() {
        pool_address.unwrap()
    } else {
        DEFAULT_POOL_ADDR.to_string()
    };
    let slot0 = call_slot0(pool_address.clone(), max_resp, cycles).await?;
    let observation = call_observation(pool_address.clone(), slot0.2, max_resp, cycles).await?;
    Ok(CandidPrice {
        sqrt_price_x96: slot0.0.to_string(),
        observation_index: slot0.2,
        block_timestamp: observation.0,
    })
}

#[query]
fn debug_get_prices_length() -> u64 {
    prices_length()
}
#[query]
fn debug_get_price(idx: u64) -> Option<CandidPrice> {
    price(idx).map(|v| v.to_candid())
}
#[query]
fn debug_last_price_timestamp_by_day() -> u32 {
    round_timestamp(last_price().unwrap().block_timestamp, 24 * 60 * 60)
}
#[query]
fn debug_last_price_timestamp_by_hour() -> u32 {
    round_timestamp(last_price().unwrap().block_timestamp, 60 * 60)
}
#[query]
fn debug_last_price_timestamp_by_five_minites() -> u32 {
    round_timestamp(last_price().unwrap().block_timestamp, 5 * 60)
}
#[query]
fn debug_price_index(timestamp: u32) -> Option<u64> {
    PRICE_INDEXES.with(|vals| vals.borrow().get(&timestamp).cloned())
}
