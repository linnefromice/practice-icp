use std::str::FromStr;

use candid::{candid_method, Principal};
use ic_cdk::{query, update};
use ic_web3::{
    ic::KeyInfo,
    types::{Address, TransactionParameters, U256},
};

use crate::{
    call_get_last_4week_realized_volatility, call_get_last_day_realized_volatility,
    env::EcdsaKeyEnvs,
    eth::{eth_gas_price, eth_tx_count, generate_web3_client},
    store::{
        get_call_canister_args, get_chain_id, get_oracle_address, get_rpc_url, get_target_canister,
        key_name, set_call_canister_args, set_chain_id, set_key_name, set_oracle_address,
        set_rpc_url, set_target_canister, timer_id,
    },
    sync_state, sync_state_internal,
    types::CallCanisterArgs,
    utils::{default_derivation_key, ethereum_address},
    MAX_RESP_TO_READ_SCALAR, MAX_RESP_TO_SEND_TX, PRECISION_FOR_ORACLE,
};

#[query]
#[candid_method(query)]
fn debug_get_target_canister() -> String {
    get_target_canister().to_string()
}
#[update]
#[candid_method(update)]
fn debug_set_target_canister(value: String) {
    set_target_canister(Principal::from_text(value).unwrap())
}
#[query]
#[candid_method(query)]
fn debug_get_call_canister_args() -> CallCanisterArgs {
    get_call_canister_args()
}
#[update]
#[candid_method(update)]
fn debug_set_call_canister_args(
    data_resource_canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    back_terms: Option<u8>,
) {
    set_call_canister_args(CallCanisterArgs {
        data_resource_canister_id,
        token0_decimals,
        token1_decimals,
        precision,
        back_terms,
    })
}
#[query]
#[candid_method(query)]
fn debug_get_rpc_url() -> String {
    get_rpc_url()
}
#[update]
#[candid_method(update)]
fn debug_set_rpc_url(rpc_url: String) {
    set_rpc_url(rpc_url)
}
#[query]
#[candid_method(query)]
fn debug_get_chain_id() -> u64 {
    get_chain_id()
}
#[update]
#[candid_method(update)]
fn debug_set_chain_id(chain_id: u64) {
    set_chain_id(chain_id)
}
#[query]
#[candid_method(query)]
fn debug_get_oracle_address() -> String {
    get_oracle_address()
}
#[update]
#[candid_method(update)]
fn debug_set_oracle_address(oracle_address: String) {
    set_oracle_address(oracle_address)
}
#[query]
#[candid_method(query)]
fn debug_get_key_name() -> String {
    key_name()
}

#[update]
#[candid_method(update)]
fn debug_set_ecdsa_key_for_local() {
    set_key_name(EcdsaKeyEnvs::LocalDevelopment)
}
#[update]
#[candid_method(update)]
fn debug_set_ecdsa_key_for_test() {
    set_key_name(EcdsaKeyEnvs::Test)
}
#[update]
#[candid_method(update)]
fn debug_set_ecdsa_key_for_prod() {
    set_key_name(EcdsaKeyEnvs::Production)
}

#[update]
#[candid_method(update)]
fn debug_stop_task() {
    let task_timer_id = timer_id();
    ic_cdk_timers::clear_timer(task_timer_id);
    ic_cdk::println!("stop task: timer_id={:?}", task_timer_id);
}

#[update]
#[candid_method(update)]
async fn debug_call_balance_of_native() -> Result<String, String> {
    let w3 = generate_web3_client(Some(MAX_RESP_TO_READ_SCALAR))
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
#[candid_method(update)]
async fn debug_call_transfer_native(to: String, value: u64) -> Result<String, String> {
    let canister_addr = ethereum_address()
        .await
        .map_err(|e| format!("get_eth_addr failed: {}", e))?;

    let w3 = generate_web3_client(Some(MAX_RESP_TO_READ_SCALAR))
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
    let w3 = generate_web3_client(Some(MAX_RESP_TO_SEND_TX))
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let signed_tx = w3
        .accounts()
        .sign_transaction(
            tx,
            hex::encode(canister_addr),
            KeyInfo {
                derivation_path: vec![default_derivation_key()],
                key_name: key_name(),
                // ecdsa_sign_cycles: None, // for latest repo
            },
            get_chain_id(),
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
#[candid_method(update)]
async fn debug_sync_state(
    target_canister_id: String,
    data_resource_canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    back_terms: Option<u8>,
    precision_to_sync: Option<u8>,
) -> Result<String, String> {
    let target_canister_id = Principal::from_text(target_canister_id).unwrap();
    sync_state(
        target_canister_id,
        CallCanisterArgs {
            data_resource_canister_id,
            token0_decimals,
            token1_decimals,
            precision,
            back_terms,
        },
        precision_to_sync.unwrap_or(PRECISION_FOR_ORACLE),
    )
    .await
}

#[update]
#[candid_method(update)]
async fn debug_sync_state_internal(
    value: u128,
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>,
) -> Result<String, String> {
    sync_state_internal(
        value,
        gas_coefficient_molecule,
        gas_coefficient_denominator,
        gas_limit,
    )
    .await
}

#[update]
#[candid_method(update)]
async fn debug_call_get_last_4week_realized_volatility(
    target_canister_id: String,
    data_resource_canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    back_terms: Option<u8>,
) -> Result<String, String> {
    let target_canister_id = Principal::from_text(target_canister_id).unwrap();
    call_get_last_4week_realized_volatility(
        target_canister_id,
        CallCanisterArgs {
            data_resource_canister_id,
            token0_decimals,
            token1_decimals,
            precision,
            back_terms,
        },
    )
    .await
}

#[update]
#[candid_method(update)]
async fn debug_call_get_last_day_realized_volatility(
    target_canister_id: String,
    data_resource_canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    back_terms: Option<u8>,
) -> Result<String, String> {
    let target_canister_id = Principal::from_text(target_canister_id).unwrap();
    call_get_last_day_realized_volatility(
        target_canister_id,
        CallCanisterArgs {
            data_resource_canister_id,
            token0_decimals,
            token1_decimals,
            precision,
            back_terms,
        },
    )
    .await
}
