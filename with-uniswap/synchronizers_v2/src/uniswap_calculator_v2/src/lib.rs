mod debug;
mod types;
mod utils;

use candid::Principal;
use ic_cdk::api::call;
use ic_cdk_macros::update;
use ic_web3::types::U256;
use types::CandidPrice;
use utils::calculate_realized_volatility;

#[update]
async fn sync_realized_volatility(
    canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    from: Option<u32>,
    to: Option<u32>,
) -> Result<String, String> {
    let canister_id = Principal::from_text(canister_id).unwrap();
    let result = calculate_realized_volatility_for_prices(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        from,
        to,
    )
    .await?;
    Ok(result.to_string())
}

async fn calculate_realized_volatility_for_prices(
    canister_id: Principal,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    from: Option<u32>,
    to: Option<u32>,
) -> Result<f64, String> {
    let exchange_rates = calculate_exchange_rates_for_prices(
        canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        from,
        to,
    )
    .await?;

    let rv = calculate_realized_volatility(&exchange_rates);

    Ok(rv)
}

async fn calculate_exchange_rates_for_prices(
    canister_id: Principal,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    from: Option<u32>,
    to: Option<u32>,
) -> Result<Vec<U256>, String> {
    let prices = call_prices(canister_id, from, to).await?;
    let mut exchange_rates = Vec::with_capacity(prices.len());
    for price in prices {
        let sqrt_price_x96 = U256::from_dec_str(&price.sqrt_price_x96).map_err(|e| {
            format!(
                "Error parsing sqrt_price_x96: {:?} for price: {:?}",
                e, price
            )
        })?;

        let exchange_rate = utils::calculate_exchange_rate(
            sqrt_price_x96,
            token0_decimals,
            token1_decimals,
            precision,
        );
        exchange_rates.push(exchange_rate);
    }
    Ok(exchange_rates)
}

async fn call_prices(
    canister_id: Principal,
    from: Option<u32>,
    to: Option<u32>,
) -> Result<Vec<CandidPrice>, String> {
    call::call::<_, (Vec<CandidPrice>,)>(canister_id, "get_prices", (from, to))
        .await
        .map(|v| v.0)
        .map_err(|e| format!("Error calling get_prices: {:?}", e))
}
