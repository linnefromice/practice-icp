use std::ops::{Div, Mul};

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
