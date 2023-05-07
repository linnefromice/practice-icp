use std::ops::{Div, Mul};

use ic_cdk::api::time;
use ic_web3::types::U256;

fn pow10(n: U256) -> U256 {
    U256::from(10).pow(n)
}

pub fn calculate_exchange_rate(
    sqrt_price_x96: U256,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
) -> U256 {
    let token0_decimals = U256::from(token0_decimals);
    let token1_decimals = U256::from(token1_decimals);
    let precision = U256::from(precision);
    sqrt_price_x96
        .mul(sqrt_price_x96)
        .mul(pow10(precision))
        .div(U256::from(2).pow(U256::from(192)))
        .mul(pow10(token0_decimals))
        .div(pow10(token1_decimals))
}

// https://www.realvol.com/VolFormula.htm
pub fn calculate_realized_volatility(prices: &[U256]) -> f64 {
    let prices_len = prices.len();

    // Calculate the log-arithmic returns
    let mut all_squared_r: Vec<f64> = Vec::with_capacity(prices_len - 1);
    for i in 1..prices_len {
        let pt = prices[i].as_u128() as f64; // TODO: consider ovewflow
        let pt_minus_1 = prices[i - 1].as_u128() as f64; // TODO: consider ovewflow
        let r = (pt / pt_minus_1).ln();
        let r_squared = r.mul(r);
        all_squared_r.push(r_squared);
    }

    // Calculate the realized volatility
    let sum_of_squared_r = all_squared_r.iter().sum::<f64>();
    (sum_of_squared_r / (prices_len - 1) as f64).sqrt() * 100.0
}

pub fn current_time_sec() -> u32 {
    (time() / (1000 * 1000000)) as u32
}

pub fn round_timestamp(timestamp: u32, unit: u32) -> u32 {
    timestamp / unit * unit
}
