use crate::{
    call_observation, call_slot0,
    constants::INDEXED_TIME_UNIT_BY_SEC,
    last_price, pool_address, price, price_index, prices_length, round_timestamp, rpc_url,
    save_prices,
    types::{CandidObservation, CandidPrice, CandidSlot0},
    TIMER_ID,
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
    pool_addr: Option<String>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidPrice, String> {
    let pool_addr = if pool_addr.is_some() {
        pool_addr.unwrap()
    } else {
        pool_address()
    };
    let slot0 = call_slot0(pool_addr.clone(), max_resp, cycles).await?;
    let observation = call_observation(pool_addr.clone(), slot0.2, max_resp, cycles).await?;
    Ok(CandidPrice {
        sqrt_price_x96: slot0.0.to_string(),
        observation_index: slot0.2,
        block_timestamp: observation.0,
    })
}
#[update]
async fn debug_save_prices(
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<(CandidPrice, Option<u32>), String> {
    save_prices(max_resp, cycles)
        .await
        .map(|(price, index)| (price.to_candid(), index))
}

#[query]
fn debug_get_rpc_url() -> String {
    rpc_url()
}
#[query]
fn debug_get_pool_address() -> String {
    pool_address()
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
fn debug_last_price_timestamp_by_hour() -> u32 {
    round_timestamp(last_price().unwrap().block_timestamp, 60 * 60)
}
#[query]
fn debug_last_price_timestamp_by_indexed_time_unit() -> u32 {
    round_timestamp(
        last_price().unwrap().block_timestamp,
        INDEXED_TIME_UNIT_BY_SEC,
    )
}
#[query]
fn debug_price_index(timestamp: u32) -> Option<u64> {
    price_index(timestamp)
}
#[query]
fn debug_get_price_indexes() -> Vec<(u32, u64)> {
    let mut result = Vec::new();
    let mut last_indexed_time = debug_last_price_timestamp_by_indexed_time_unit();
    loop {
        let price_index = price_index(last_indexed_time);
        result.push((last_indexed_time, price_index.unwrap()));
        if let None | Some(0) = price_index {
            break;
        }
        last_indexed_time -= INDEXED_TIME_UNIT_BY_SEC;
    }
    result
}
#[update]
fn debug_stop_periodic_save_prices() {
    let timer_id = TIMER_ID.with(|value| *value.borrow());
    ic_cdk_timers::clear_timer(timer_id);
}
