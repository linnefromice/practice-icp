mod constants;
mod debug;
mod types;
mod utils;

use constants::{DEFAULT_POOL_ADDR, UNISWAPV3_POOL_ABI};
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{query, update};
use ic_web3::contract::Options;
use std::{cell::RefCell, collections::HashMap};
use types::{Observation, Price, Slot0};
use utils::{generate_uniswapv3pool_client, generate_web3_client};

thread_local! {
    static PRICE_INDEXES: RefCell<HashMap<u32,u64>> = RefCell::new(HashMap::new());
    static PRICES: RefCell<Vec<Price>> = RefCell::default();
}

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
async fn save_prices(max_resp: Option<u64>, cycles: Option<u64>) -> Result<(), String> {
    let pool_address = DEFAULT_POOL_ADDR.to_string();
    let slot0 = call_slot0(pool_address.clone(), max_resp, cycles).await?;
    let observation = call_observation(pool_address.clone(), slot0.2, max_resp, cycles).await?;

    // validations
    let last_price = last_price();
    if let Some(value) = last_price {
        if value.block_timestamp == observation.0 {
            return Err(format!(
                "Already fetched: timestamp={}",
                value.block_timestamp
            ));
        }
    }

    // save price
    let price = Price {
        sqrt_price_x96: slot0.0,
        observation_index: slot0.2,
        block_timestamp: observation.0,
    };
    PRICES.with(|prices| {
        prices.borrow_mut().push(price.clone());
    });

    // save index
    let unit = 5 * 60;
    let rounded_timestamp = round_timestamp(price.block_timestamp, unit);
    let last_index = PRICE_INDEXES.with(|vals| vals.borrow().get(&rounded_timestamp).cloned());
    if last_index.is_some() {
        return Ok(());
    }
    let saved_latest_index = prices_length() - 1;
    PRICE_INDEXES.with(|vals| {
        vals.borrow_mut()
            .insert(rounded_timestamp, saved_latest_index);
    });

    Ok(())
}

async fn call_slot0(
    pool_address: String,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<Slot0, String> {
    let max_resp = if max_resp.is_some() {
        max_resp
    } else {
        Some(700) // default
    };
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
    let max_resp = if max_resp.is_some() {
        max_resp
    } else {
        Some(550) // default
    };
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

fn last_price() -> Option<Price> {
    PRICES.with(|val| val.borrow().last().cloned())
}
fn prices_length() -> u64 {
    PRICES.with(|val| val.borrow().len()) as u64
}
fn price(idx: u64) -> Option<Price> {
    PRICES.with(|val| val.borrow().get(idx as usize).cloned())
}
fn round_timestamp(timestamp: u32, unit: u32) -> u32 {
    timestamp / unit * unit
}
