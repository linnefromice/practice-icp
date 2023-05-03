mod constants;
mod debug;
mod store;
mod types;
mod utils;

use constants::{
    BASE_MAX_RESP_BYTES_FOR_HEADER, DEFAULT_FETCH_INTERVAL_BY_SEC,
    MAX_RESP_BYTES_TO_CALL_OBSERVATION, MAX_RESP_BYTES_TO_CALL_SLOT0, UNISWAPV3_POOL_ABI,
};
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{init, query, update};
use ic_web3::{
    contract::Options,
    types::{BlockId, BlockNumber, U64},
};
use store::{
    add_price, get_pool_address, last_price, prices, set_pool_address, set_rpc_url, set_timer_id,
};
use types::{CandidPrice, Observation, Price, Slot0};
use utils::{generate_uniswapv3pool_client, generate_web3_client};

#[init]
fn init(url: String, pool_addr: String) {
    set_rpc_url(url);
    set_pool_address(pool_addr);
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
async fn periodic_save_prices(
    interval_secs: Option<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) {
    let interval_secs = if let Some(value) = interval_secs {
        value
    } else {
        DEFAULT_FETCH_INTERVAL_BY_SEC
    };
    let interval = std::time::Duration::from_secs(interval_secs);
    let timer_id = ic_cdk_timers::set_timer_interval(interval, move || {
        ic_cdk::spawn(async move {
            let res = save_prices(None, max_resp, cycles).await;
            match res {
                Ok((price, indexed_timestamp)) => {
                    ic_cdk::println!("price: {:?}", price);
                    ic_cdk::println!("indexed_timestamp: {:?}", indexed_timestamp);
                }
                Err(msg) => ic_cdk::println!("error msg: {:?}", msg),
            }
        });
    });
    set_timer_id(timer_id);
}

#[query]
fn get_prices() -> Vec<CandidPrice> {
    let prices = prices();
    prices.iter().map(|price| price.to_candid()).collect()
}

#[update]
async fn bulk_save_prices(block_numbers: Vec<u64>, max_resp: Option<u64>, cycles: Option<u64>) {
    ic_cdk::println!("START: bulk save prices");
    for bn in &block_numbers {
        let res = save_prices(Some(*bn), max_resp, cycles).await;
        match res {
            Ok((price, indexed_timestamp)) => {
                ic_cdk::println!("price: {:?}", price);
                ic_cdk::println!("indexed_timestamp: {:?}", indexed_timestamp);
            }
            Err(msg) => ic_cdk::println!("error msg: {:?}", msg),
        }
    }
    ic_cdk::println!("END: bulk save prices");
}

async fn save_prices(
    block_number: Option<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<(Price, Option<u32>), String> {
    let pool_addr = get_pool_address();
    let slot0 = call_slot0(pool_addr.clone(), block_number, max_resp, cycles).await?;
    let observation =
        call_observation(pool_addr.clone(), slot0.2, block_number, max_resp, cycles).await?;

    let price = Price {
        sqrt_price_x96: slot0.0,
        observation_index: slot0.2,
        block_timestamp: observation.0,
    };
    update_states(price)
}
fn update_states(price: Price) -> Result<(Price, Option<u32>), String> {
    // validations
    let last_price = last_price();
    if let Some(value) = last_price {
        if value.block_timestamp == price.block_timestamp {
            return Err(format!(
                "Already fetched: timestamp={}",
                value.block_timestamp
            ));
        }
    }

    // save price
    add_price(price.clone());

    Ok((price, None))
}

async fn call_slot0(
    pool_address: String,
    block_number: Option<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<Slot0, String> {
    let max_resp = if max_resp.is_some() {
        max_resp
    } else {
        Some(BASE_MAX_RESP_BYTES_FOR_HEADER + MAX_RESP_BYTES_TO_CALL_SLOT0) // default
    };
    let w3 = generate_web3_client(max_resp, cycles)?;
    let contract = generate_uniswapv3pool_client(w3, pool_address.as_str(), UNISWAPV3_POOL_ABI)?;
    let block_number =
        block_number.map(|value| BlockId::from(BlockNumber::Number(U64::from(value))));
    contract
        .query("slot0", (), None, Options::default(), block_number)
        .await
        .map_err(|e| format!("query contract error: {}", e))
}

async fn call_observation(
    pool_address: String,
    observation_idx: u16,
    block_number: Option<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<Observation, String> {
    let max_resp = if max_resp.is_some() {
        max_resp
    } else {
        Some(BASE_MAX_RESP_BYTES_FOR_HEADER + MAX_RESP_BYTES_TO_CALL_OBSERVATION)
        // default
    };
    let w3 = generate_web3_client(max_resp, cycles)?;
    let contract = generate_uniswapv3pool_client(w3, pool_address.as_str(), UNISWAPV3_POOL_ABI)?;
    let block_number =
        block_number.map(|value| BlockId::from(BlockNumber::Number(U64::from(value))));
    contract
        .query(
            "observations",
            observation_idx,
            None,
            Options::default(),
            block_number,
        )
        .await
        .map_err(|e| format!("query contract error: {}", e))
}
