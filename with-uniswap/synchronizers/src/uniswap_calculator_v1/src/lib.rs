mod debug;
mod utils;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::call, init, update};
use ic_web3::types::U256;
use std::{
    cell::RefCell,
    ops::{Add, Div},
};
use utils::calculate_exchange_rate;

thread_local! {
    static READER_CANISTERS: RefCell<Vec<Principal>> = RefCell::default();
    static TOKEN0_DECIMALS: RefCell<u8> = RefCell::default();
    static TOKEN1_DECIMALS: RefCell<u8> = RefCell::default();
}

#[init]
fn init(token0_decimals: u8, token1_decimals: u8) {
    TOKEN0_DECIMALS.with(|value| *value.borrow_mut() = token0_decimals);
    TOKEN1_DECIMALS.with(|value| *value.borrow_mut() = token1_decimals);
}

#[update]
fn register_reader(reader: Principal) {
    READER_CANISTERS.with(|value| {
        let mut readers = value.borrow_mut();
        readers.push(reader);
    });
}

#[update]
async fn calculate_average_exchange_rate(
    from: Option<u32>,
    to: Option<u32>,
    precision: u8,
) -> String {
    let nested_prices = get_prices_from_all_registered(from, to).await;
    let prices = nested_prices.into_iter().flatten().collect::<Vec<_>>();
    let count = prices.len();
    let token0_decimals = token0_decimals();
    let token1_decimals = token1_decimals();
    let mut sum = U256::zero();
    for price in prices {
        let sqrt_price_x96 = U256::from_dec_str(&price.sqrt_price_x96).unwrap();
        sum = sum.add(calculate_exchange_rate(
            sqrt_price_x96,
            token0_decimals,
            token1_decimals,
            precision,
        ));
    }
    let average = sum.div(U256::from(count));
    average.to_string()
}

fn token0_decimals() -> u8 {
    TOKEN0_DECIMALS.with(|value| *value.borrow())
}
fn token1_decimals() -> u8 {
    TOKEN0_DECIMALS.with(|value| *value.borrow())
}

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct CandidPrice {
    pub sqrt_price_x96: String,
    pub observation_index: u16,
    pub block_timestamp: u32,
}
async fn get_prices_from_all_registered(
    from: Option<u32>,
    to: Option<u32>,
) -> Vec<Vec<CandidPrice>> {
    let canisters = READER_CANISTERS.with(|value| value.borrow().clone());
    let mut results = Vec::<Vec<CandidPrice>>::new();
    for canister in canisters {
        let result = call_get_prices_in_reader(canister, from, to).await;
        results.push(result);
    }
    results
}
async fn call_get_prices_in_reader(
    canister_id: Principal,
    from: Option<u32>,
    to: Option<u32>,
) -> Vec<CandidPrice> {
    call::call::<_, (Vec<CandidPrice>,)>(canister_id, "get_prices", (from, to))
        .await
        .unwrap()
        .0
}
