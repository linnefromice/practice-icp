mod constants;
mod debug;
mod types;
mod utils;

use constants::{DEFAULT_FETCH_INTERVAL_BY_SEC, UNISWAPV3_POOL_ABI};
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{init, query, update};
use ic_cdk_timers::TimerId;
use ic_web3::{
    contract::Options,
    types::{BlockId, BlockNumber, U64},
};
use std::cell::RefCell;
use types::{CandidPrice, Observation, Price, Slot0};
use utils::{generate_uniswapv3pool_client, generate_web3_client};

thread_local! {
    static PRICES: RefCell<Vec<Price>> = RefCell::default();
    static TIMER_ID: RefCell<TimerId> = RefCell::default();
    static RPC_URL: RefCell<String> = RefCell::default();
    static POOL_ADDRESS: RefCell<String> = RefCell::default();
}

#[init]
fn init(url: String, pool_addr: String) {
    RPC_URL.with(|value| *value.borrow_mut() = url);
    POOL_ADDRESS.with(|value| *value.borrow_mut() = pool_addr);
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
    TIMER_ID.with(|value| *value.borrow_mut() = timer_id);
}

#[query]
fn get_prices() -> Vec<CandidPrice> {
    let prices = PRICES.with(|prices| prices.borrow().clone());
    prices.iter().map(|price| price.to_candid()).collect()
}

async fn save_prices(
    block_number: Option<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<(Price, Option<u32>), String> {
    let pool_addr = pool_address();
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
    PRICES.with(|prices| {
        prices.borrow_mut().push(price.clone());
    });

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
        Some(700) // default
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
        Some(550) // default
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

fn rpc_url() -> String {
    RPC_URL.with(|value| (value.borrow()).clone())
}
fn pool_address() -> String {
    POOL_ADDRESS.with(|value| (value.borrow()).clone())
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
