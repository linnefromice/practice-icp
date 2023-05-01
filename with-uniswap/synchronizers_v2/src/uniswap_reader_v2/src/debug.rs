use crate::{
    call_observation, call_slot0, pool_address, price, prices_length, rpc_url, save_prices,
    types::{CandidObservation, CandidPrice, CandidSlot0},
    utils::generate_web3_client,
    TIMER_ID,
};
use ic_cdk_macros::{query, update};

#[update]
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
async fn debug_fetch_price(
    pool_addr: Option<String>,
    block_number: Option<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidPrice, String> {
    let pool_addr = if pool_addr.is_some() {
        pool_addr.unwrap()
    } else {
        pool_address()
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
async fn debug_save_prices(
    block_number: Option<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<(CandidPrice, Option<u32>), String> {
    save_prices(block_number, max_resp, cycles)
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
#[update]
fn debug_stop_periodic_save_prices() {
    let timer_id = TIMER_ID.with(|value| *value.borrow());
    ic_cdk_timers::clear_timer(timer_id);
}
#[update]
async fn debug_fetch_block_number() -> Result<u64, String> {
    let w3 = generate_web3_client(Some(300), None)?;
    match w3.eth().block_number().await {
        Ok(v) => Ok(v.as_u64()),
        Err(e) => Err(e.to_string()),
    }
}
