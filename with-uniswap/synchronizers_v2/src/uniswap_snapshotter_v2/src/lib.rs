mod constants;
mod debug;
mod store;
mod types;
mod utils;

use common::{types::CandidPrice, utils::round_timestamp};
use constants::{
    BASE_MAX_RESP_BYTES_FOR_HEADER, DEFAULT_FETCH_INTERVAL_BY_SEC,
    DEFAULT_PRICE_INDEX_INTERVAL_SEC, MAX_RESP_BYTES_TO_CALL_OBSERVATION,
    MAX_RESP_BYTES_TO_CALL_SLOT0, UNISWAPV3_POOL_ABI,
};
use ic_cdk::api::{
    management_canister::http_request::{HttpResponse, TransformArgs},
    time,
};
use ic_cdk_macros::{query, update};
use ic_web3::{
    contract::Options,
    types::{BlockId, BlockNumber, U64},
};
use store::{
    add_price, get_closest_high_price_index, get_closest_low_price_index,
    get_from_past_synced_timestamp, get_from_synced_timestamp, get_pool_address, get_price_index,
    get_price_index_interval_sec, insert_price_index, last_price, prices, prices_length,
    set_pool_address, set_price_index_interval_sec, set_rpc_url, set_timer_id,
};
use types::{Observation, Price, Slot0};
use utils::{generate_uniswapv3pool_client, generate_web3_client};

use crate::store::{set_from_past_synced_timestamp, set_from_synced_timestamp};

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
fn setup(url: String, pool_addr: String, price_index_interval_secs: Option<u32>) {
    set_rpc_url(url);
    set_pool_address(pool_addr);

    let price_index_interval_secs = if let Some(value) = price_index_interval_secs {
        value
    } else {
        DEFAULT_PRICE_INDEX_INTERVAL_SEC
    };
    set_price_index_interval_sec(price_index_interval_secs);
}

#[update]
async fn set_task(interval_secs: Option<u32>, max_resp: Option<u64>, cycles: Option<u64>) {
    // TODO: check to call bulk_save_prices
    // if !get_initialized_past_prices() {
    //     ic_cdk::println!("Not initialized yet");
    //     return;
    // }
    // TODO: check timer_id registered
    let interval_secs = interval_secs.unwrap_or(DEFAULT_FETCH_INTERVAL_BY_SEC);
    let price_index_interval_secs = get_price_index_interval_sec();
    if interval_secs >= price_index_interval_secs {
        ic_cdk::println!("interval_secs must be less than price_index_interval_secs: interval_secs={}, price_index_interval_secs={}", interval_secs, price_index_interval_secs);
        return;
    }
    // TODO: check synced for past prices

    let current_time_sec = (time() / (1000 * 1000000)) as u32;
    let from_synced_timestamp = round_timestamp(current_time_sec, interval_secs) + interval_secs;
    let delay = from_synced_timestamp - current_time_sec;
    let interval = std::time::Duration::from_secs(interval_secs as u64);
    ic_cdk::println!("START: set_timer for set_timer_interval");
    ic_cdk::println!("{}", current_time_sec);
    ic_cdk_timers::set_timer(std::time::Duration::from_secs(delay as u64), move || {
        ic_cdk::println!("START: set_timer_interval for set periodic_save_prices");
        ic_cdk::println!("{}", (time() / (1000 * 1000000)));

        // set scheduled executions timer for 2nd and later
        let timer_id = ic_cdk_timers::set_timer_interval(interval, move || {
            ic_cdk::spawn(async move {
                ic_cdk::println!("START: execute save_prices by timer");
                ic_cdk::println!("{}", (time() / (1000 * 1000000)));
                let res = save_prices(None, max_resp, cycles).await;
                match res {
                    Ok((price, indexed_timestamp)) => {
                        ic_cdk::println!("price: {:?}", price);
                        ic_cdk::println!("indexed_timestamp: {:?}", indexed_timestamp);
                    }
                    Err(msg) => ic_cdk::println!("error msg: {:?}", msg),
                }
                ic_cdk::println!("FINISH: execute save_prices by timer");
            });
        });
        set_timer_id(timer_id);

        // for 1st
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

        ic_cdk::println!("FINISH: set_timer_interval for set periodic_save_prices");
    });
    ic_cdk::println!("FINISH: set_timer for set_timer_interval");
    set_from_synced_timestamp(from_synced_timestamp);
}

#[query]
fn get_prices(from: Option<u32>, to: Option<u32>) -> Result<Vec<CandidPrice>, String> {
    let price_data = prices();
    // TODO: impl validations to check data counts
    let filtered_prices = match (from, to) {
        (Some(from_ts), Some(to_ts)) => {
            let from = get_closest_high_price_index(from_ts, to_ts)?;
            let to = get_closest_low_price_index(to_ts, from_ts)?;
            price_data[(from as usize)..(to as usize)].to_vec()
        }
        (Some(from_ts), None) => {
            let from = get_closest_high_price_index(from_ts, u32::MAX)?; // temp: u32::MAX
            let (_, slice_from_index) = price_data.split_at(from as usize);
            slice_from_index.to_vec()
        }
        (None, Some(to_ts)) => {
            let to = get_closest_low_price_index(to_ts, u32::MIN)?; // temp: u32::MIN
            let (slice_up_to_index, _) = price_data.split_at(to as usize);
            slice_up_to_index.to_vec()
        }
        _ => price_data,
    };
    Ok(filtered_prices
        .iter()
        .map(|price| price.to_candid())
        .collect())
}

#[query]
fn get_price_indexes(from_past: bool) -> Vec<(u32, u64)> {
    let mut result = Vec::new();
    let mut last_indexed_time = get_last_price_timestamp_by_indexed_time_unit();
    let from_synced_timestamp = if from_past {
        get_from_past_synced_timestamp()
    } else {
        get_from_synced_timestamp()
    };
    if from_synced_timestamp == 0 {
        return result;
    }
    loop {
        let price_index: Option<u64> = get_price_index(last_indexed_time);
        result.push((last_indexed_time, price_index.unwrap_or(0)));
        if last_indexed_time <= from_synced_timestamp {
            break;
        }
        last_indexed_time -= get_price_index_interval_sec();
    }
    result
}
#[query]
fn get_filtered_price_indexes(from_past: bool) -> Vec<(u32, u64)> {
    let result = get_price_indexes(from_past);
    let mut filtered: Vec<(u32, u64)> = result
        .iter()
        .filter(|(_, price_index)| *price_index > 0)
        .cloned()
        .collect();
    if filtered.len() <= 1 {
        return filtered;
    };
    if from_past {
        filtered.push(*result.last().unwrap());
    }
    filtered
}

#[update]
async fn bulk_save_prices(
    block_numbers: Vec<u64>,
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<(), String> {
    ic_cdk::println!("START: bulk save prices");
    let splitted = block_numbers.split_first();
    if splitted.is_none() {
        return Err("block_numbers must have at least one element".to_string());
    }
    let (first_bn, rest_bns) = splitted.unwrap();

    // save to store for past 1st
    let res = save_prices(Some(*first_bn), max_resp, cycles).await;
    match res {
        Ok((price, indexed_timestamp)) => {
            ic_cdk::println!("price: {:?}", price);
            ic_cdk::println!("indexed_timestamp: {:?}", indexed_timestamp);
            set_from_past_synced_timestamp(round_timestamp(
                price.block_timestamp,
                get_price_index_interval_sec(),
            ));
        }
        Err(msg) => ic_cdk::println!("error msg: {:?}", msg),
    }

    // for rest
    for bn in rest_bns {
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
    Ok(())
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
    if let Some(value) = last_price.clone() {
        if value.block_timestamp == price.block_timestamp {
            return Err(format!(
                "Already fetched: timestamp={}",
                value.block_timestamp
            ));
        }
    }

    // save price
    add_price(price.clone());

    // save index
    let price_index_interval_sec = get_price_index_interval_sec();
    let rounded_timestamp = round_timestamp(price.block_timestamp, price_index_interval_sec);
    let last_index = get_price_index(rounded_timestamp);
    if last_index.is_some() {
        return Ok((price, None));
    }
    let saved_latest_index = prices_length() - 1;
    insert_price_index(rounded_timestamp, saved_latest_index);
    //// consider remaining price between
    // if let Some(value) = last_price {
    //     let last_rounded_timestamp =
    //         round_timestamp(value.block_timestamp, price_index_interval_sec);
    //     let mut price_index_timestamp_in_middle = last_rounded_timestamp;
    //     loop {
    //         price_index_timestamp_in_middle += price_index_interval_sec;
    //         if price_index_timestamp_in_middle >= rounded_timestamp {
    //             break;
    //         }
    //         insert_price_index(price_index_timestamp_in_middle, saved_latest_index - 1);
    //     }
    // }

    Ok((price, Some(rounded_timestamp)))
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

fn get_last_price_timestamp_by_indexed_time_unit() -> u32 {
    round_timestamp(
        last_price().unwrap().block_timestamp,
        get_price_index_interval_sec(),
    )
}
