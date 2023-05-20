use crate::{
    call_observation, call_slot0,
    get_last_price_timestamp_by_indexed_time_unit, save_prices,
    store::{
        get_from_past_synced_timestamp, get_from_synced_timestamp, get_pool_address, get_rpc_url,
        get_timer_id, price, prices, prices_length, clear_prices, clear_price_indexes,
    },
    types::{CandidObservation, CandidPrice, CandidSlot0},
    fetch_latest_block_number,
};
use candid::candid_method;
use ic_cdk_macros::{query, update};

#[update]
#[candid_method(update)]
async fn debug_fetch_slot0(
    pool_address: String,
    block_number: Option<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidSlot0, String> {
    let result = call_slot0(pool_address, block_number, max_resp, cycles).await;
    result.map(|v| v.to_candid())
}

#[update]
#[candid_method(update)]
async fn debug_fetch_observation(
    pool_address: String,
    observation_idx: u16,
    block_number: Option<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidObservation, String> {
    let result = call_observation(
        pool_address,
        observation_idx,
        block_number,
        max_resp,
        cycles,
    )
    .await;
    result.map(|v| v.to_candid())
}

#[update]
#[candid_method(update)]
async fn debug_fetch_price(
    pool_addr: Option<String>,
    block_number: Option<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidPrice, String> {
    let pool_addr = if pool_addr.is_some() {
        pool_addr.unwrap()
    } else {
        get_pool_address()
    };
    let slot0 = call_slot0(pool_addr.clone(), block_number, max_resp, cycles).await?;
    let observation =
        call_observation(pool_addr.clone(), slot0.2, block_number, max_resp, cycles).await?;
    Ok(CandidPrice {
        sqrt_price_x96: slot0.0.to_string(),
        observation_index: slot0.2,
        block_timestamp: observation.0,
    })
}
#[update]
#[candid_method(update)]
async fn debug_save_prices(
    block_number: Option<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<(CandidPrice, Option<u32>), String> {
    save_prices(block_number, max_resp, cycles)
        .await
        .map(|(price, index)| (price.to_candid(), index))
}
#[update]
#[candid_method(update)]
async fn debug_clear_prices() {
    clear_prices();
    clear_price_indexes();
}

#[query]
#[candid_method(query)]
fn debug_get_rpc_url() -> String {
    get_rpc_url()
}
#[query]
#[candid_method(query)]
fn debug_get_pool_address() -> String {
    get_pool_address()
}
#[query]
#[candid_method(query)]
fn debug_get_from_synced_timestamp() -> u32 {
    get_from_synced_timestamp()
}
#[query]
#[candid_method(query)]
fn debug_get_from_past_synced_timestamp() -> u32 {
    get_from_past_synced_timestamp()
}
#[query]
#[candid_method(query)]
fn debug_get_prices_length() -> u64 {
    prices_length()
}
#[query]
#[candid_method(query)]
fn debug_get_price(idx: u64) -> Option<CandidPrice> {
    price(idx).map(|v| v.to_candid())
}
#[query]
#[candid_method(query)]
fn debug_get_prices() -> Vec<CandidPrice> {
    prices().iter().map(|v| v.to_candid()).collect()
}
#[update]
#[candid_method(query)]
fn debug_stop_periodic_save_prices() {
    ic_cdk_timers::clear_timer(get_timer_id());
}
#[query]
#[candid_method(query)]
fn debug_last_price_timestamp_by_indexed_time_unit() -> u32 {
    get_last_price_timestamp_by_indexed_time_unit()
}
#[update]
#[candid_method(update)]
async fn debug_fetch_block_number() -> Result<u64, String> {
    fetch_latest_block_number().await
}
