mod debug;
mod types;
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
