mod debug;
mod env;
mod eth;
mod store;
mod types;
mod utils;

use std::time::Duration;

use candid::{candid_method, Principal};
use eth::{generate_web3_client, sign};
use ic_cdk::{
    api::management_canister::http_request::{HttpResponse, TransformArgs},
    query, update,
};
use ic_web3::{transports::ICHttp, types::SignedTransaction, Web3};
use store::{
    get_call_canister_args, get_oracle_address, get_target_canister, set_call_canister_args,
    set_chain_id, set_oracle_address, set_rpc_url, set_target_canister, set_timer_id,
};
use types::CallCanisterArgs;

const ORACLE_ABI: &[u8] = include_bytes!("../../abi/Oracle.json");
const ORACLE_FUNC_NAME: &str = "updateState";

// parameters
const MAX_RESP_TO_READ_SCALAR: u64 = 300;
const MAX_RESP_TO_SEND_TX: u64 = 500;
const TASK_INTERVAL_SECS: u64 = 60 * 60;
const PRECISION_FOR_ORACLE: u8 = 18;

#[update]
#[candid_method(update)]
async fn get_ethereum_address() -> String {
    match utils::ethereum_address().await {
        Ok(v) => format!("0x{}", hex::encode(v)),
        Err(msg) => msg,
    }
}

#[query]
#[candid_method(query)]
fn transform(response: TransformArgs) -> HttpResponse {
    let res = response.response;
    // remove headers
    HttpResponse {
        status: res.status,
        headers: Vec::default(),
        body: res.body,
    }
}

#[update]
#[candid_method(update)]
async fn setup(target_canister_id: String, rpc_url: String, chain_id: u64, oracle_addr: String) {
    set_target_canister(Principal::from_text(target_canister_id).unwrap());
    set_call_canister_args(CallCanisterArgs::default()); // TODO
    set_rpc_url(rpc_url);
    set_chain_id(chain_id);
    set_oracle_address(oracle_addr);
    let timer_id =
        ic_cdk_timers::set_timer_interval(Duration::from_secs(TASK_INTERVAL_SECS), || {
            ic_cdk::spawn(async {
                match sync_state(
                    get_target_canister(),
                    get_call_canister_args(),
                    PRECISION_FOR_ORACLE,
                )
                .await
                {
                    Ok(msg) => ic_cdk::println!("ok: {:?}", msg),
                    Err(msg) => ic_cdk::println!("err: {:?}", msg),
                }
            });
        });
    set_timer_id(timer_id);
    ic_cdk::println!("start task: timer_id={:?}", timer_id);
}

async fn sync_state(
    canister_id: Principal,
    call_args: CallCanisterArgs,
    precision_to_sync: u8,
) -> Result<String, String> {
    let result: Result<String, String> =
        call_get_last_day_realized_volatility(canister_id, call_args).await;
    if let Err(msg) = result {
        return Err(format!("error msg by inter-canister call: {:?}", msg));
    }
    let parsed_result = result.unwrap().parse::<f64>();
    if let Err(msg) = parsed_result {
        return Err(format!("error msg by parsing result: {:?}", msg));
    }
    let value = (parsed_result.unwrap() * 10u64.pow(precision_to_sync as u32) as f64) as u128;
    match sync_state_internal(value, None, None, None).await {
        Ok(hash) => Ok(format!("txhash: {:?}", hash)),
        Err(msg) => Err(format!("error msg: {:?}", msg)),
    }
}

async fn sync_state_internal(
    value: u128,
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>,
) -> Result<String, String> {
    let w3 = generate_web3_client(Some(MAX_RESP_TO_READ_SCALAR))?;
    let signed_tx = sync_state_signed_tx_internal(
        w3,
        value,
        gas_coefficient_molecule,
        gas_coefficient_denominator,
        gas_limit,
    )
    .await?;

    let w3 = generate_web3_client(Some(MAX_RESP_TO_SEND_TX))?;
    match w3
        .eth()
        .send_raw_transaction(signed_tx.raw_transaction)
        .await
    {
        Ok(v) => Ok(format!("0x{}", hex::encode(v))),
        Err(msg) => Err(format!("send_raw_transaction failed: {}", msg)),
    }
}

async fn sync_state_signed_tx_internal(
    w3: Web3<ICHttp>,
    value: u128,
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>,
) -> Result<SignedTransaction, String> {
    sign(
        w3,
        &get_oracle_address(),
        ORACLE_ABI,
        ORACLE_FUNC_NAME,
        value,
        if gas_coefficient_molecule.is_some() && gas_coefficient_denominator.is_some() {
            Some((
                gas_coefficient_molecule.unwrap(),
                gas_coefficient_denominator.unwrap(),
            ))
        } else {
            None
        },
        gas_limit,
    )
    .await
}

async fn call_get_last_4week_realized_volatility(
    canister_id: Principal,
    call_args: CallCanisterArgs,
) -> Result<String, String> {
    let res = ic_cdk::api::call::call::<_, (Result<String, String>,)>(
        canister_id,
        "get_last_4week_realized_volatility",
        (
            call_args.data_resource_canister_id,
            call_args.token0_decimals,
            call_args.token1_decimals,
            call_args.precision,
            call_args.back_terms,
        ),
    )
    .await
    .map_err(|e| format!("Error calling get_last_4week_realized_volatility: {:?}", e))?;
    res.0
        .map_err(|e| format!("Error calling get_last_4week_realized_volatility: {:?}", e))
}

async fn call_get_last_day_realized_volatility(
    canister_id: Principal,
    call_args: CallCanisterArgs,
) -> Result<String, String> {
    let res = ic_cdk::api::call::call::<_, (Result<String, String>,)>(
        canister_id,
        "get_last_day_realized_volatility",
        (
            call_args.data_resource_canister_id,
            call_args.token0_decimals,
            call_args.token1_decimals,
            call_args.precision,
            call_args.back_terms,
        ),
    )
    .await
    .map_err(|e| format!("Error calling get_last_day_realized_volatility: {:?}", e))?;
    res.0
        .map_err(|e| format!("Error calling get_last_day_realized_volatility: {:?}", e))
}
