mod constants;
mod types;
mod utils;

use constants::UNISWAPV3_POOL_ABI;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{query, update};
use ic_web3::contract::Options;
use types::{CandidObservation, CandidSlot0, Observation, Slot0};
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

#[update]
async fn get_exchange_rate(
    pool_address: String,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidSlot0, String> {
    let w3 = generate_web3_client(max_resp, cycles)?;
    let contract = generate_uniswapv3pool_client(w3, pool_address.as_str(), UNISWAPV3_POOL_ABI)?;
    let result: Result<Slot0, String> = contract
        .query("slot0", (), None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e));
    let slot0 = result.unwrap();
    Ok(slot0.to_candid())
}

#[update]
async fn get_observation(
    pool_address: String,
    observation_idx: u16,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidObservation, String> {
    let w3 = generate_web3_client(max_resp, cycles)?;
    let contract = generate_uniswapv3pool_client(w3, pool_address.as_str(), UNISWAPV3_POOL_ABI)?;
    let result: Result<Observation, String> = contract
        .query(
            "observations",
            observation_idx,
            None,
            Options::default(),
            None,
        )
        .await
        .map_err(|e| format!("query contract error: {}", e));
    let observation = result.unwrap();
    Ok(observation.to_candid())
}
