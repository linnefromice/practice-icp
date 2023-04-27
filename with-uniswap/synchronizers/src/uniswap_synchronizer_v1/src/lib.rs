mod constants;
mod debug;
mod types;
mod utils;

use candid::Principal;
use constants::DEFAULT_FETCH_INTERVAL_BY_SEC;
use ic_cdk::{
    api::{
        call,
        management_canister::http_request::{HttpResponse, TransformArgs},
        time,
    },
    init, query, spawn, update,
};
use ic_cdk_timers::TimerId;
use ic_web3::{
    transports::ICHttp,
    types::{Address, SignedTransaction, U256},
    Web3,
};
use std::cell::RefCell;
use types::ExchangeRate;
use utils::{generate_web3_client, sign};

const KEY_NAME: &str = "dfx_test_key";
const ORACLE_ABI: &[u8] = include_bytes!("../../abi/Oracle.json");

thread_local! {
    static RPC_URL: RefCell<String>  = RefCell::default();
    static CHAIN_ID: RefCell<u64>  = RefCell::default();
    static ORACLE_ADDRESS: RefCell<String> = RefCell::default();
    static INDEXER_CANISTER_ID: RefCell<Principal> = RefCell::new(Principal::anonymous());
    static SYNCING_FROM: RefCell<u32> = RefCell::default();
    static TIMER_ID: RefCell<TimerId> = RefCell::default();
}

#[init]
fn init(url: String, chain_id: u64, oracle_addr: String, indexer: Principal) {
    RPC_URL.with(|value| *value.borrow_mut() = url);
    CHAIN_ID.with(|value| *value.borrow_mut() = chain_id);
    ORACLE_ADDRESS.with(|value| *value.borrow_mut() = oracle_addr);
    INDEXER_CANISTER_ID.with(|value| *value.borrow_mut() = indexer);
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
fn periodic_sync_state(
    interval_secs: Option<u64>,
    initial_from: Option<u32>,
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>,
) {
    let interval_sec = if let Some(value) = interval_secs {
        value
    } else {
        DEFAULT_FETCH_INTERVAL_BY_SEC
    };

    // set `from` for call_calculate_average_exchange_rate
    let initial_start_time = if let Some(value) = initial_from {
        value
    } else {
        let current_time_sec = time() / (1000 * 1000000); // nano sec -> sec
        (current_time_sec / interval_sec * interval_sec - interval_sec) as u32 // (curent - 1) period
    };
    SYNCING_FROM.with(|value| *value.borrow_mut() = initial_start_time);

    let interval = std::time::Duration::from_secs(interval_sec);
    let timer_id = ic_cdk_timers::set_timer_interval(interval, move || {
        spawn(async move {
            let canister_id = indexer_canister_id();
            let from = syncing_from();
            let to = from + interval_sec as u32;
            let res_calculator =
                call_calculate_average_exchange_rate(canister_id, Some(from), Some(to), 0).await;
            let rate = U256::from_dec_str(&res_calculator);
            if let Err(msg) = rate {
                ic_cdk::println!(
                    "parse error from call_calculate_average_exchange_rate: {}",
                    msg
                );
                return;
            }
            let res_eth = sync_state_internal(
                ExchangeRate {
                    rate: rate.unwrap().as_u128(),
                    from_time: from as u128,
                    to_time: to as u128,
                },
                gas_coefficient_molecule,
                gas_coefficient_denominator,
                gas_limit,
            )
            .await;
            match res_eth {
                Ok(hash) => {
                    SYNCING_FROM.with(|value| *value.borrow_mut() = to);
                    ic_cdk::println!(
                        "txhash: {:?}, synced from: {:?}, syncing_from: {:?}",
                        hash,
                        from,
                        to
                    );
                }
                Err(msg) => ic_cdk::println!("error msg: {:?}", msg),
            }
        });
    });
    TIMER_ID.with(|value| *value.borrow_mut() = timer_id);
    ic_cdk::println!("[START] periodic_sync_state");
}

fn rpc_url() -> String {
    RPC_URL.with(|value| (value.borrow()).clone())
}
fn chain_id() -> u64 {
    CHAIN_ID.with(|value| *value.borrow())
}
fn oracle_address() -> String {
    ORACLE_ADDRESS.with(|value| (value.borrow()).clone())
}
fn indexer_canister_id() -> Principal {
    INDEXER_CANISTER_ID.with(|value| *value.borrow())
}
fn syncing_from() -> u32 {
    SYNCING_FROM.with(|value| *value.borrow())
}
fn key_name() -> &'static str {
    KEY_NAME
}

async fn call_calculate_average_exchange_rate(
    canister_id: Principal,
    from: Option<u32>,
    to: Option<u32>,
    precision: u8,
) -> String {
    call::call::<_, (String,)>(
        canister_id,
        "calculate_average_exchange_rate",
        (from, to, precision),
    )
    .await
    .unwrap()
    .0
}

async fn eth_tx_count(w3: Web3<ICHttp>, addr: Address) -> Result<U256, String> {
    w3.eth()
        .transaction_count(addr, None)
        .await
        .map_err(|e| format!("get tx count error: {}", e))
}

async fn eth_gas_price(w3: Web3<ICHttp>) -> Result<U256, String> {
    w3.eth()
        .gas_price()
        .await
        .map_err(|e| format!("get gas_price error: {}", e))
}

async fn sync_state_internal(
    exchange_rate: ExchangeRate,
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>,
) -> Result<String, String> {
    let w3 = generate_web3_client(Some(300), None)?;
    let signed_tx = sync_state_signed_tx_internal(
        w3,
        exchange_rate,
        gas_coefficient_molecule,
        gas_coefficient_denominator,
        gas_limit,
    )
    .await?;

    let w3 = generate_web3_client(Some(500), None)?;
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
    exchange_rate: ExchangeRate,
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>,
) -> Result<SignedTransaction, String> {
    sign(
        w3,
        &oracle_address(),
        ORACLE_ABI,
        "updateState",
        exchange_rate,
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
