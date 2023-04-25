use crate::{
    call_observation, call_slot0,
    types::{CandidObservation, CandidSlot0},
};
use ic_cdk_macros::update;

#[update]
async fn debug_get_slot0(
    pool_address: String,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidSlot0, String> {
    let result = call_slot0(pool_address, max_resp, cycles).await;
    result.map(|v| v.to_candid())
}

#[update]
async fn debug_get_observation(
    pool_address: String,
    observation_idx: u16,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<CandidObservation, String> {
    let result = call_observation(pool_address, observation_idx, max_resp, cycles).await;
    result.map(|v| v.to_candid())
}
