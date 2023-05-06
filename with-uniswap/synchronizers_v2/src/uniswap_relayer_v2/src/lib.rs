mod debug;
mod env;
mod eth;
mod store;
mod utils;

use std::time::Duration;

use candid::candid_method;
use eth::{generate_web3_client, sign};
use ic_cdk::{
    api::{
        call::CallResult,
        management_canister::http_request::{HttpResponse, TransformArgs},
    },
    query, update,
};
use ic_web3::{transports::ICHttp, types::SignedTransaction, Web3};
use store::{
    hhi_canister, mapper, oracle_address, set_chain_id, set_hhi_canister, set_mapper,
    set_oracle_address, set_rpc_url, set_timer_id,
};

const ORACLE_ABI: &[u8] = include_bytes!("../../abi/Oracle.json");
const ORACLE_FUNC_NAME: &str = "updateState";

// parameters
const MAX_RESP_TO_READ_SCALAR: u64 = 300;
const MAX_RESP_TO_SEND_TX: u64 = 500;
const TASK_INTERVAL_SECS: u64 = 60 * 60;
const TOP_N_FOR_HHI: u64 = 100;

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
async fn setup(
    hhi_canister_id: String,
    mapper_canister_id: String,
    rpc_url: String,
    chain_id: u64,
    oracle_addr: String,
) {
    set_hhi_canister(hhi_canister_id);
    set_mapper(mapper_canister_id);
    set_rpc_url(rpc_url);
    set_chain_id(chain_id);
    set_oracle_address(oracle_addr);
    let timer_id =
        ic_cdk_timers::set_timer_interval(Duration::from_secs(TASK_INTERVAL_SECS), || {
            ic_cdk::spawn(async {
                let result: CallResult<(u128,)> = ic_cdk::api::call::call(
                    hhi_canister(),
                    "hhi_of_top_n",
                    (mapper(), TOP_N_FOR_HHI),
                )
                .await;
                if let Err(msg) = result {
                    ic_cdk::println!("error msg by calling hhi_of_top_n: {:?}", msg);
                    return;
                }
                match sync_state_internal(result.unwrap().0, None, None, None).await {
                    Ok(hash) => ic_cdk::println!("txhash: {:?}", hash),
                    Err(msg) => ic_cdk::println!("error msg: {:?}", msg),
                }
            });
        });
    set_timer_id(timer_id);
    ic_cdk::println!("start task: timer_id={:?}", timer_id);
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
        &oracle_address(),
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
