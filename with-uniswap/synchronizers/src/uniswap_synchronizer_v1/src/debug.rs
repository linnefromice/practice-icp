use std::str::FromStr;

use ic_cdk::{api::time, query, update};
use ic_web3::{
    ic::KeyInfo,
    types::{Address, TransactionParameters, U256},
};

use crate::{
    call_calculate_average_exchange_rate, chain_id,
    constants::DEFAULT_FETCH_INTERVAL_BY_SEC,
    eth_gas_price, eth_tx_count, indexer_canister_id, oracle_address, rpc_url, sync_state_internal,
    syncing_from,
    types::ExchangeRate,
    utils::{
        default_derivation_key, ethereum_address, generate_web3_client, public_key,
        to_ethereum_address,
    },
    KEY_NAME, TIMER_ID,
};

#[query]
fn debug_get_rpc_url() -> String {
    rpc_url()
}
#[query]
fn debug_get_oracle_address() -> String {
    oracle_address()
}
#[query]
fn debug_get_indexer_canister_id() -> String {
    indexer_canister_id().to_string()
}
#[query]
fn debug_get_syncing_from() -> u32 {
    syncing_from()
}
#[update]
async fn debug_ethereum_address_and_public_key() -> Result<(String, String), String> {
    let pub_key = public_key().await?;
    let address = to_ethereum_address(pub_key.clone())?;
    Ok((
        format!("0x{}", hex::encode(address)),
        format!("0x{}", hex::encode(pub_key)),
    ))
}
#[update]
async fn debug_call_calculate_average_exchange_rate(
    from: Option<u32>,
    to: Option<u32>,
    precision: u8,
) -> String {
    let canister_id = indexer_canister_id();
    call_calculate_average_exchange_rate(canister_id, from, to, precision).await
}
#[update]
async fn debug_call_rpc_tx_count() -> Result<String, String> {
    let w3 = generate_web3_client(Some(300), None)
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let canister_addr = ethereum_address()
        .await
        .map_err(|e| format!("get_eth_addr failed: {}", e))?;
    let tx_count = eth_tx_count(w3, canister_addr).await?;
    Ok(tx_count.to_string())
}
#[update]
async fn debug_call_rpc_gas_price() -> Result<String, String> {
    let w3 = generate_web3_client(Some(300), None)
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let gas_price = eth_gas_price(w3).await?;
    Ok(gas_price.to_string())
}
#[update]
async fn debug_call_balance_of_native() -> Result<String, String> {
    let w3 = generate_web3_client(Some(300), None)
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let canister_addr = ethereum_address()
        .await
        .map_err(|e| format!("get_eth_addr failed: {}", e))?;
    let balance = w3
        .eth()
        .balance(canister_addr, None)
        .await
        .map_err(|e| format!("get balance failed: {}", e))?;
    Ok(balance.to_string())
}
#[update]
async fn debug_call_transfer_native(to: String, value: u64) -> Result<String, String> {
    let canister_addr = ethereum_address()
        .await
        .map_err(|e| format!("get_eth_addr failed: {}", e))?;

    let w3 = generate_web3_client(Some(300), None)
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let tx_count = eth_tx_count(w3.clone(), canister_addr).await?;
    let gas_price = eth_gas_price(w3.clone()).await?;

    let to = Address::from_str(&to).unwrap();
    let tx = TransactionParameters {
        to: Some(to),
        nonce: Some(tx_count),
        value: U256::from(value),
        gas_price: Some(gas_price),
        gas: U256::from(21000),
        ..Default::default()
    };
    let w3 = generate_web3_client(Some(500), None)
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let signed_tx = w3
        .accounts()
        .sign_transaction(
            tx,
            hex::encode(canister_addr),
            KeyInfo {
                derivation_path: vec![default_derivation_key()],
                key_name: KEY_NAME.to_string(),
            },
            chain_id(),
        )
        .await
        .map_err(|e| format!("sign tx error: {}", e))?;
    match w3
        .eth()
        .send_raw_transaction(signed_tx.raw_transaction)
        .await
    {
        Ok(txhash) => {
            ic_cdk::println!("txhash: {}", hex::encode(txhash.0));
            Ok(hex::encode(txhash.0))
        }
        Err(e) => Err(e.to_string()),
    }
}

#[update]
async fn debug_sync_state(
    rate: u128,
    from_time: u128, // TODO: u256
    to_time: u128,   // TODO: u256
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>,
) -> Result<String, String> {
    sync_state_internal(
        ExchangeRate {
            rate,
            from_time,
            to_time,
        },
        gas_coefficient_molecule,
        gas_coefficient_denominator,
        gas_limit,
    )
    .await
}

#[update]
fn debug_stop_periodic_task() {
    let timer_id = TIMER_ID.with(|value| *value.borrow());
    ic_cdk_timers::clear_timer(timer_id);
}
#[query]
fn debug_current_icp_time() -> u64 {
    ic_cdk::api::time()
}
#[query]
fn debug_calculate_from_from_current_icp_time() -> u64 {
    let interval_sec = DEFAULT_FETCH_INTERVAL_BY_SEC;
    let current_time_sec = time() / (1000 * 1000000);
    current_time_sec / interval_sec * interval_sec - interval_sec
}
