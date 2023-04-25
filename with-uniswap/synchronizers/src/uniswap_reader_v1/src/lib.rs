mod constants;
mod debug;
mod types;
mod utils;

use constants::{DEFAULT_FETCH_INTERVAL_BY_SEC, INDEXED_TIME_UNIT_BY_SEC, UNISWAPV3_POOL_ABI};
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{init, query, update};
use ic_cdk_timers::TimerId;
use ic_web3::contract::Options;
use std::{cell::RefCell, collections::HashMap};
use types::{CandidPrice, Observation, Price, Slot0};
use utils::{generate_uniswapv3pool_client, generate_web3_client};

thread_local! {
    static PRICE_INDEXES: RefCell<HashMap<u32,u64>> = RefCell::new(HashMap::new());
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
            let res = save_prices(max_resp, cycles).await;
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
fn get_prices(from: Option<u32>, to: Option<u32>) -> Vec<CandidPrice> {
    let prices = PRICES.with(|prices| {
        let prices = prices.borrow().clone();
        // TODO: impl validations to check data counts
        match (from, to) {
            (Some(from_ts), Some(to_ts)) => {
                let from = price_index(from_ts).unwrap();
                let to = price_index(to_ts).unwrap();
                prices[(from as usize)..(to as usize)].to_vec()
            }
            (Some(from_ts), None) => {
                let from = price_index(from_ts).unwrap();
                let (_, slice_from_index) = prices.split_at(from as usize);
                slice_from_index.to_vec()
            }
            (None, Some(to_ts)) => {
                let to = price_index(to_ts).unwrap();
                let (slice_up_to_index, _) = prices.split_at(to as usize);
                slice_up_to_index.to_vec()
            }
            _ => prices,
        }
    });
    prices.iter().map(|price| price.to_candid()).collect()
}

async fn save_prices(
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<(Price, Option<u32>), String> {
    let pool_addr = pool_address();
    let slot0 = call_slot0(pool_addr.clone(), max_resp, cycles).await?;
    let observation = call_observation(pool_addr.clone(), slot0.2, max_resp, cycles).await?;

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

    // save index
    let rounded_timestamp = round_timestamp(price.block_timestamp, INDEXED_TIME_UNIT_BY_SEC);
    let last_index = PRICE_INDEXES.with(|vals| vals.borrow().get(&rounded_timestamp).cloned());
    if last_index.is_some() {
        return Ok((price, None));
    }
    let saved_latest_index = prices_length() - 1;
    PRICE_INDEXES.with(|vals| {
        vals.borrow_mut()
            .insert(rounded_timestamp, saved_latest_index);
    });

    Ok((price, Some(rounded_timestamp)))
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
fn price_index(timestamp: u32) -> Option<u64> {
    PRICE_INDEXES.with(|vals| vals.borrow().get(&timestamp).cloned())
}
fn round_timestamp(timestamp: u32, unit: u32) -> u32 {
    timestamp / unit * unit
}

#[cfg(test)]
mod tests {
    use ic_web3::types::U256;

    use super::*;

    #[test]
    fn test_get_prices() {
        // prerequisites
        let test_data = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, INDEXED_TIME_UNIT_BY_SEC - 1),
            (4, INDEXED_TIME_UNIT_BY_SEC),
            (5, INDEXED_TIME_UNIT_BY_SEC + 1),
            (6, INDEXED_TIME_UNIT_BY_SEC + 2),
            (7, INDEXED_TIME_UNIT_BY_SEC * 2 - 1),
            (8, INDEXED_TIME_UNIT_BY_SEC * 2),
            (9, INDEXED_TIME_UNIT_BY_SEC * 2 + 1),
            (10, INDEXED_TIME_UNIT_BY_SEC * 2 + 2),
            (11, INDEXED_TIME_UNIT_BY_SEC * 3 - 1),
            (12, INDEXED_TIME_UNIT_BY_SEC * 3),
        ];
        let prices = test_data
            .iter()
            .map(|(observation_index, block_timestamp)| Price {
                sqrt_price_x96: U256::from(*observation_index),
                observation_index: *observation_index,
                block_timestamp: *block_timestamp,
            })
            .collect::<Vec<Price>>();
        for price in prices.clone() {
            update_states(price.clone()).unwrap();
        }
        assert_eq!(price_index(INDEXED_TIME_UNIT_BY_SEC).unwrap(), 4);
        assert_eq!(price_index(INDEXED_TIME_UNIT_BY_SEC * 2).unwrap(), 8);
        assert_eq!(price_index(INDEXED_TIME_UNIT_BY_SEC * 3).unwrap(), 12);

        assert_eq!(get_prices(None, None).len(), 13);
        assert_eq!(
            get_prices(Some(INDEXED_TIME_UNIT_BY_SEC * 2), None),
            vec![
                prices[8].to_candid(),
                prices[9].to_candid(),
                prices[10].to_candid(),
                prices[11].to_candid(),
                prices[12].to_candid()
            ]
        );
        assert_eq!(
            get_prices(None, Some(INDEXED_TIME_UNIT_BY_SEC * 2)),
            vec![
                prices[0].to_candid(),
                prices[1].to_candid(),
                prices[2].to_candid(),
                prices[3].to_candid(),
                prices[4].to_candid(),
                prices[5].to_candid(),
                prices[6].to_candid(),
                prices[7].to_candid()
            ]
        );
        assert_eq!(
            get_prices(
                Some(INDEXED_TIME_UNIT_BY_SEC),
                Some(INDEXED_TIME_UNIT_BY_SEC * 2)
            ),
            vec![
                prices[4].to_candid(),
                prices[5].to_candid(),
                prices[6].to_candid(),
                prices[7].to_candid()
            ]
        );
    }
}
