mod debug;
mod utils;

use candid::Principal;
use ic_cdk::{
    api::{
        call,
        management_canister::http_request::{HttpResponse, TransformArgs},
    },
    init, query,
};
use ic_web3::{
    transports::ICHttp,
    types::{Address, U256},
    Web3,
};
use std::cell::RefCell;

const KEY_NAME: &str = "dfx_test_key";

thread_local! {
    static RPC_URL: RefCell<String>  = RefCell::default();
    static CHAIN_ID: RefCell<u64>  = RefCell::default();
    static ORACLE_ADDRESS: RefCell<String> = RefCell::default();
    static INDEXER_CANISTER_ID: RefCell<Principal> = RefCell::new(Principal::anonymous());
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

fn rpc_url() -> String {
    RPC_URL.with(|value| (value.borrow()).clone())
}
fn chain_id() -> u64 {
    CHAIN_ID.with(|value| (value.borrow()).clone())
}
fn oracle_address() -> String {
    ORACLE_ADDRESS.with(|value| (value.borrow()).clone())
}
fn indexer_canister_id() -> Principal {
    INDEXER_CANISTER_ID.with(|value| *value.borrow())
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
