mod constants;
mod debug;
mod types;
mod utils;

use candid::CandidType;
use constants::UNISWAPV3_POOL_ABI;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{query, update};
use ic_web3::contract::Options;
use types::{Observation, Slot0};
use utils::{generate_uniswapv3pool_client, generate_web3_client};

#[query]
fn transform(response: TransformArgs) -> HttpResponse {
    let res = response.response;
    HttpResponse {
        status: res.status,
        headers: Vec::default(),
        body: res.body,
    }
}

#[derive(CandidType)]
pub struct ResponseLatestExchangeRate {
    pub sqrt_price_x96: String,
    pub observation_index: u16,
    pub block_timestamp: u32,
}
#[update]
async fn get_latest_exchange_rate(
    pool_address: String,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<ResponseLatestExchangeRate, String> {
    let slot0 = call_slot0(pool_address.clone(), max_resp, cycles).await?;
    let observation = call_observation(pool_address.clone(), slot0.2, max_resp, cycles).await?;
    Ok(ResponseLatestExchangeRate {
        sqrt_price_x96: slot0.0.to_string(),
        observation_index: slot0.2,
        block_timestamp: observation.0,
    })
}

async fn call_slot0(
    pool_address: String,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<Slot0, String> {
    let w3 = generate_web3_client(max_resp, cycles)?;
    let contract = generate_uniswapv3pool_client(w3, pool_address.as_str(), UNISWAPV3_POOL_ABI)?;
    contract
        .query("slot0", (), None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e))
}

async fn call_observation(
    pool_address: String,
    observation_idx: u16,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<Observation, String> {
    let w3 = generate_web3_client(max_resp, cycles)?;
    let contract = generate_uniswapv3pool_client(w3, pool_address.as_str(), UNISWAPV3_POOL_ABI)?;
    contract
        .query(
            "observations",
            observation_idx,
            None,
            Options::default(),
            None,
        )
        .await
        .map_err(|e| format!("query contract error: {}", e))
}
