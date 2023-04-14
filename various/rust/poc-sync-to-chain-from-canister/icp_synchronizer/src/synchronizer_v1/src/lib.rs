mod utils;

use std::{str::FromStr, sync::atomic::{AtomicU64, Ordering}};
use candid::CandidType;
use ic_cdk::{query, update, api::management_canister::http_request::{TransformArgs, HttpResponse}};
use ic_web3::{Web3, types::{Address, SignedTransaction, U64}, transports::ICHttp, contract::{Contract, Options, tokens::Tokenize}, ic::{get_eth_addr, KeyInfo}};
use utils::{get_rpc_endpoint, KEY_NAME, default_derivation_key, get_public_key, pubkey_to_address, generate_web3_client, CHAIN_ID};

// Oracle
const ORACLE_ADDR: &'static str = "af974dfd33cb1105710eddbb8f30f1ba3c994da1"; // remove 0x
const ORACLE_ABI: &[u8] = include_bytes!("../../abi/OracleV1.json");

#[derive(CandidType)]
struct AccountInfo {
    pub address: String,
    pub pub_key: String
}

static LATEST_ROUND: AtomicU64 = AtomicU64::new(0);

#[query]
fn transform(response: TransformArgs) -> HttpResponse {
    response.response
}

#[update]
async fn periodic_update_state() {
    let default_timer_interval_secs = 5;
    let interval = std::time::Duration::from_secs(default_timer_interval_secs);
    ic_cdk::println!("Starting a periodic task with interval {interval:?}");

    ic_cdk_timers::set_timer_interval(interval, || {
        let latest_round = LATEST_ROUND.load(Ordering::Relaxed);
        // let _ = update_state( // temp
        //     latest_round as u128,
        //     latest_round as i128,
        //     latest_round as u128,
        //     latest_round as u128
        // );
        let updated_round = LATEST_ROUND.fetch_add(1, Ordering::Relaxed);
        ic_cdk::println!("round_id is {updated_round}");
    });
}

#[update]
async fn update_state(
    id: u128, // TODO: u256
    answer: i128, // TODO: i256
    started_at: u128, // TODO: u256
    updated_at: u128, // TODO: u256
) -> Result<String, String> {
    let w3 = generate_web3_client()
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let signed_tx = update_state_signed_tx_internal(
        w3.clone(),
        id,
        answer,
        started_at,
        updated_at
    ).await?;
    match w3.eth().send_raw_transaction(signed_tx.raw_transaction).await {
        Ok(v) => Ok(format!("0x{}", hex::encode(v))),
        Err(msg) => Err(format!("send_raw_transaction failed: {}", msg))
    }
}

async fn update_state_signed_tx_internal(
    w3: Web3<ICHttp>,
    id: u128, // TODO: u256
    answer: i128, // TODO: i256
    started_at: u128, // TODO: u256
    updated_at: u128, // TODO: u256
) -> Result<SignedTransaction, String> {
    sign(
        w3,
        &ORACLE_ADDR,
        &ORACLE_ABI,
        &"updateState",
        (id,answer,started_at,updated_at,)
    ).await
}

async fn sign(
    w3: Web3<ICHttp>,
    contract_addr: &str,
    abi: &[u8],
    func: &str,
    params: impl Tokenize,
) -> Result<SignedTransaction, String> {
    let contract = generate_contract_client(w3.clone(), contract_addr, abi)
        .map_err(|e| format!("generate_contract_client failed: {}", e))?;
    let canister_addr = get_eth_addr(None, None, KEY_NAME.to_string()).await
        .map_err(|e| format!("get_eth_addr failed: {}", e))?;

    let tx_count = w3.eth()
        .transaction_count(canister_addr, None)
        .await
        .map_err(|e| format!("get tx count error: {}", e))?;
    let gas_price = w3.eth()
        .gas_price()
        .await
        .map_err(|e| format!("get gas_price error: {}", e))?;
    let options = Options::with(|op| {
        op.nonce = Some(tx_count);
        op.gas_price = Some(gas_price);
        op.transaction_type = Some(U64::from(2)) // EIP1559_TX_ID
    });

    match contract.sign(
        func,
        params,
        options,
        hex::encode(canister_addr),
        KeyInfo { derivation_path: vec![default_derivation_key()], key_name: KEY_NAME.to_string() },
        CHAIN_ID // TODO: switch chain
    ).await {
        Ok(v) => Ok(v),
        Err(msg) => Err(format!("sign failed: {}", msg))
    }
}

fn generate_contract_client(w3: Web3<ICHttp>, contract_addr: &str, abi: &[u8]) -> Result<Contract<ICHttp>, String> {
    let contract_address = Address::from_str(contract_addr).unwrap();
    Contract::from_json(
        w3.eth(),
        contract_address,
        abi
    ).map_err(|e| format!("init contract failed: {}", e))
}

#[derive(CandidType)]
struct CandidSignedTransaction {
    pub message_hash: String,
    pub v: u64,
    pub r: String,
    pub s: String,
    pub raw_transaction: String,
    pub transaction_hash: String,
}
#[update]
async fn debug_oracle_latest_round_id() -> Result<u128, String> {
    let w3 = generate_web3_client()
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let contract = generate_contract_client(w3, ORACLE_ADDR, ORACLE_ABI)?;
    
    contract
        .query("latestRoundId", (), None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e))
}
#[update]
async fn debug_update_state_signed_tx(
    id: u128, // TODO: u256
    answer: i128, // TODO: i256
    started_at: u128, // TODO: u256
    updated_at: u128, // TODO: u256
) -> Result<CandidSignedTransaction, String> {
    let w3 = generate_web3_client()
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    match update_state_signed_tx_internal(
        w3.clone(),
        id,
        answer,
        started_at,
        updated_at,
    ).await {
        Ok(signed_tx) =>
            Ok(CandidSignedTransaction {
                message_hash: format!("0x{}", hex::encode(signed_tx.message_hash)),
                v: signed_tx.v,
                r: format!("0x{}", hex::encode(signed_tx.r)),
                s: format!("0x{}", hex::encode(signed_tx.s)),
                raw_transaction: format!("0x{}", hex::encode(signed_tx.raw_transaction.0)),
                transaction_hash: format!("0x{}", hex::encode(signed_tx.transaction_hash)),
            }),
        Err(msg) => Err(msg)
    }
}

#[query]
fn debug_rpc_endpoint() -> String {
    get_rpc_endpoint()
}
#[update]
async fn debug_account_info() -> Result<AccountInfo, String> {
    let pub_key = get_public_key(None, vec![default_derivation_key()], KEY_NAME.to_string()).await;
    if let Err(msg) = pub_key { return Err(msg) };
    let pub_key = pub_key.unwrap();

    let addr = pubkey_to_address(&pub_key);
    if let Err(msg) = addr { return Err(msg) };
    let addr = addr.unwrap();

    return Ok(AccountInfo {
        address: format!("0x{}", hex::encode(addr)),
        pub_key: format!("0x{}", hex::encode(pub_key)),
    })
}