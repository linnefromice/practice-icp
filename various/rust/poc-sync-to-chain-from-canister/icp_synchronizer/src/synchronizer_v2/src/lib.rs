mod types;
mod utils;

use std::{str::FromStr, ops::{Mul, Div}, cell::RefCell};
use candid::CandidType;
use ic_cdk::{query, update, api::{management_canister::http_request::{TransformArgs, HttpResponse}, self}, spawn};
use ic_web3::{Web3, types::{Address, SignedTransaction, U64, U256, TransactionParameters}, transports::ICHttp, contract::{Contract, Options, tokens::{Tokenize, Tokenizable}}, ic::{get_eth_addr, KeyInfo}, ethabi::Token};
use ic_cdk_timers::TimerId;
use utils::{get_rpc_endpoint, KEY_NAME, default_derivation_key, get_public_key, pubkey_to_address, generate_web3_client, CHAIN_ID};
use types::{AccountInfo, Round};

// Oracle
const DEFAULT_ORACLE_ADDR: &'static str = "8E7d7C9dD03f76CCaDEB1729C6B0F644145837Cb"; // remove 0x
const ORACLE_ABI: &[u8] = include_bytes!("../../abi/OracleV2.json");

thread_local! {
    static LATEST_ROUND_ID: RefCell<u128> = RefCell::default();
    static ROUNDS: RefCell<Vec<Round>> = RefCell::default();
    static SYNCED_LATEST_ROUND_ID: RefCell<u128> = RefCell::default();
     // for debug
    static TIMER_ID: RefCell<TimerId> = RefCell::default();
    static ORACLE_ADDR: RefCell<String> = RefCell::new(String::from(DEFAULT_ORACLE_ADDR))
}

#[query]
fn transform(response: TransformArgs) -> HttpResponse {
    response.response
}

#[update]
fn update_state(answer: i128) -> Round {
    let timestamp = api::time();
    update_state_internal(answer, timestamp, timestamp)
}
fn update_state_internal(answer: i128, started_at: u64, updated_at: u64) -> Round {
    let incremented_round_id = LATEST_ROUND_ID.with(|val| {
        let mut mut_ref = val.borrow_mut();
        *mut_ref += 1;
        mut_ref.clone()
    });
    let round = Round {
        round_id: incremented_round_id,
        answer,
        started_at,
        updated_at
    };
    ROUNDS.with(|rounds| rounds.borrow_mut().push(round.clone()));
    round
}

#[update]
async fn periodic_sync_state(
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>
) {
    periodic_sync_state_internal(
        60,
        1,
        gas_coefficient_molecule,
        gas_coefficient_denominator,
        gas_limit
    )
}

fn periodic_sync_state_internal(
    interval_secs: u64,
    max_run_unit: u128,
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>
) {
    let interval = std::time::Duration::from_secs(interval_secs);
    let max_run_unit_owned = std::sync::Arc::new(max_run_unit);

    let timer_id = ic_cdk_timers::set_timer_interval(interval, move || {
        ic_cdk::println!("[START] Synchronization");
        let synced_latest_round_id = get_synced_latest_round_id();
        let latest_round_id = get_latest_round_id();
        if synced_latest_round_id == latest_round_id {
            ic_cdk::println!("Already synced: round_id is {:?}", synced_latest_round_id);
            return
        }
        let not_synced = latest_round_id - synced_latest_round_id;
        let run_unit = if not_synced > *max_run_unit_owned { *max_run_unit_owned } else { not_synced };

        // TODO: use `sync_state_bulk`
        ic_cdk::println!("syncing rounds: from_id = {:?}, run_unit = {:?}", synced_latest_round_id + 1, run_unit);
        spawn(async move {
            let res = sync_state_bulk(
                synced_latest_round_id + 1,
                run_unit,
                gas_coefficient_molecule,
                gas_coefficient_denominator,
                gas_limit
            ).await;
            match res {
                Ok(hash) => ic_cdk::println!("txhash: {:?}", hash),
                Err(msg) => ic_cdk::println!("error msg: {:?}", msg),
            }
        });

        SYNCED_LATEST_ROUND_ID.with(|value| *value.borrow_mut() += run_unit);
        ic_cdk::println!("[FINISH] Synchronization");
    });

    TIMER_ID.with(|value| *value.borrow_mut() = timer_id);
}

fn get_latest_round_id() -> u128 {
    LATEST_ROUND_ID.with(|value| (*value.borrow()).clone())
}
fn get_round(idx: u128) -> Round {
    ROUNDS.with(|rounds| {
        let rounds = rounds.borrow();
        rounds[idx as usize].clone()
    })
}
fn get_synced_latest_round_id() -> u128 {
    SYNCED_LATEST_ROUND_ID.with(|value| (*value.borrow()).clone())
}

#[update]
async fn sync_state(
    id: u128, // TODO: u256
    answer: i128, // TODO: i256
    started_at: u128, // TODO: u256
    updated_at: u128, // TODO: u256
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>
) -> Result<String, String> {
    let w3 = generate_web3_client()
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let signed_tx = sync_state_signed_tx_internal(
        w3.clone(),
        id,
        answer,
        started_at,
        updated_at,
        gas_coefficient_molecule,
        gas_coefficient_denominator,
        gas_limit
    ).await?;
    match w3.eth().send_raw_transaction(signed_tx.raw_transaction).await {
        Ok(v) => Ok(format!("0x{}", hex::encode(v))),
        Err(msg) => Err(format!("send_raw_transaction failed: {}", msg))
    }
}

async fn sync_state_signed_tx_internal(
    w3: Web3<ICHttp>,
    id: u128, // TODO: u256
    answer: i128, // TODO: i256
    started_at: u128, // TODO: u256
    updated_at: u128, // TODO: u256
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>,
) -> Result<SignedTransaction, String> {
    sign(
        w3,
        &oracle_addr(),
        &ORACLE_ABI,
        &"updateState",
        (id,answer,started_at,updated_at,),
        if gas_coefficient_molecule.is_some() && gas_coefficient_denominator.is_some() { Some((gas_coefficient_molecule.unwrap(), gas_coefficient_denominator.unwrap())) } else { None },
        gas_limit
    ).await
}

#[update]
async fn sync_state_bulk(
    from_id: u128,
    count: u128,
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>
) -> Result<String, String> {
    let w3 = generate_web3_client()
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let signed_tx = sync_state_bulk_signed_tx_internal(
        w3.clone(),
        get_rounds(from_id, count),
        gas_coefficient_molecule,
        gas_coefficient_denominator,
        gas_limit
    ).await?;
    match w3.eth().send_raw_transaction(signed_tx.raw_transaction).await {
        Ok(v) => Ok(format!("0x{}", hex::encode(v))),
        Err(msg) => Err(format!("send_raw_transaction failed: {}", msg))
    }
}

async fn sync_state_bulk_signed_tx_internal(
    w3: Web3<ICHttp>,
    rounds: Vec<Round>,
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>,
) -> Result<SignedTransaction, String> {
    let tokens = rounds.iter()
        .map(|round| (*round).into_token())
        .collect::<Vec<Token>>();

    sign(
        w3,
        &oracle_addr(),
        &ORACLE_ABI,
        &"updateStates",
        Token::Array(tokens),
        if gas_coefficient_molecule.is_some() && gas_coefficient_denominator.is_some() { Some((gas_coefficient_molecule.unwrap(), gas_coefficient_denominator.unwrap())) } else { None },
        gas_limit
    ).await
}


async fn sign(
    w3: Web3<ICHttp>,
    contract_addr: &str,
    abi: &[u8],
    func: &str,
    params: impl Tokenize,
    gas_coefficient: Option<(u128, u128)>,
    gas_limit: Option<u128>,
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
        op.transaction_type = Some(U64::from(2)); // EIP1559_TX_ID
        if gas_coefficient.is_some() {
            let gas_coefficient_value = gas_coefficient.unwrap();
            op.gas_price = Some(gas_price.mul(U256::from(gas_coefficient_value.0)).div(U256::from(gas_coefficient_value.1)));
        } else {
            op.gas_price = Some(gas_price);
        }

        // temp
        if let Some(gas_limit_value) = gas_limit {
            op.gas = Some(U256::from(gas_limit_value))
        }
    });
    // TODO: consider gas_limit
    // let estimated_gas = contract.estimate_gas(func, params.clone(), canister_addr, options)
    //     .await
    //     .map_err(|e| format!("estimate_gas error: {}", e))?;
    // options.gas = Some(estimated_gas);

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

fn get_rounds(from_id: u128, count: u128) -> Vec<Round> {
    let mut result: Vec<Round> = Vec::<Round>::new();
    for i in 0..count {
        let round = get_round(from_id - 1 + i);
        result.push(round);
    }
    result
}

fn generate_contract_client(w3: Web3<ICHttp>, contract_addr: &str, abi: &[u8]) -> Result<Contract<ICHttp>, String> {
    let contract_address = Address::from_str(contract_addr).unwrap();
    Contract::from_json(
        w3.eth(),
        contract_address,
        abi
    ).map_err(|e| format!("init contract failed: {}", e))
}

fn oracle_addr() -> String {
    ORACLE_ADDR.with(|val| val.borrow().clone())
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
    let contract = generate_contract_client(w3, &oracle_addr(), ORACLE_ABI)?;
    
    contract
        .query("latestRoundId", (), None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e))
}
#[update]
async fn debug_sync_state_signed_tx(
    id: u128, // TODO: u256
    answer: i128, // TODO: i256
    started_at: u128, // TODO: u256
    updated_at: u128, // TODO: u256
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>,
) -> Result<CandidSignedTransaction, String> {
    let w3 = generate_web3_client()
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    match sync_state_signed_tx_internal(
        w3.clone(),
        id,
        answer,
        started_at,
        updated_at,
        gas_coefficient_molecule,
        gas_coefficient_denominator,
        gas_limit
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
#[update]
async fn debug_sync_state_estimate_gas(
    id: u128, // TODO: u256
    answer: i128, // TODO: i256
    started_at: u128, // TODO: u256
    updated_at: u128, // TODO: u256
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
) -> Result<String, String> {
    let gas_coefficient = if gas_coefficient_molecule.is_some() && gas_coefficient_denominator.is_some() {
        Some((gas_coefficient_molecule.unwrap(), gas_coefficient_denominator.unwrap()))
    } else {
        None
    };

    let w3 = generate_web3_client()
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let contract = generate_contract_client(w3.clone(), &oracle_addr(), &ORACLE_ABI)
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
        op.transaction_type = Some(U64::from(2)); // EIP1559_TX_ID
        if gas_coefficient.is_some() {
            let gas_coefficient_value = gas_coefficient.unwrap();
            op.gas_price = Some(gas_price.mul(U256::from(gas_coefficient_value.0)).div(U256::from(gas_coefficient_value.1)));
        } else {
            op.gas_price = Some(gas_price);
        }
    });

    let estimated_gas = contract.estimate_gas(&"updateState", (id,answer,started_at,updated_at,), canister_addr, options)
        .await
        .map_err(|e| format!("estimate_gas error: {}", e))?;
    Ok(estimated_gas.to_string())
}

#[update]
async fn debug_sync_state_bulk_signed_tx(
    from_id: u128,
    count: u128,
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>,
) -> Result<CandidSignedTransaction, String> {
    let w3 = generate_web3_client()
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    match sync_state_bulk_signed_tx_internal(
        w3.clone(),
        get_rounds(from_id, count),
        gas_coefficient_molecule,
        gas_coefficient_denominator,
        gas_limit
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

#[update]
async fn debug_balance_of_native() -> Result<String, String> {
    let w3 = generate_web3_client()
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let canister_addr = get_eth_addr(None, None, KEY_NAME.to_string())
        .await
        .map_err(|e| format!("get_eth_addr failed: {}", e))?;
    let balance = w3.eth()
        .balance(canister_addr, None)
        .await
        .map_err(|e| format!("get balance failed: {}", e))?;
    Ok(balance.to_string())
}
#[update]
async fn debug_transfer_native(to: String, value: u64) -> Result<String, String> {
    let w3 = generate_web3_client()
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
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
    let to = Address::from_str(&to).unwrap();
    let tx = TransactionParameters {
        to: Some(to),
        nonce: Some(tx_count), // remember to fetch nonce first
        value: U256::from(value),
        gas_price: Some(gas_price),
        gas: U256::from(21000),
        ..Default::default()
    };
    let signed_tx = w3.accounts()
        .sign_transaction(
            tx,
            hex::encode(canister_addr),
            KeyInfo { derivation_path: vec![default_derivation_key()], key_name: KEY_NAME.to_string() },
            CHAIN_ID
        )
        .await
        .map_err(|e| format!("sign tx error: {}", e))?;
    match w3.eth().send_raw_transaction(signed_tx.raw_transaction).await {
        Ok(txhash) => {
            ic_cdk::println!("txhash: {}", hex::encode(txhash.0));
            Ok(format!("{}", hex::encode(txhash.0)))
        },
        Err(e) => { Err(e.to_string()) },
    }
}
#[update]
async fn debug_gas_price() -> Result<String, String> {
    let w3 = generate_web3_client()
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let gas_price = w3.eth()
        .gas_price()
        .await
        .map_err(|e| format!("get gas_price error: {}", e))?;
    Ok(gas_price.to_string())
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

#[update]
fn debug_update_state(answer: i128, started_at: u64, updated_at: u64) -> Round {
    update_state_internal(answer, started_at, updated_at)
}
#[update]
fn debug_clean_state() {
    ROUNDS.with(|rounds| *rounds.borrow_mut() = Vec::new());
    LATEST_ROUND_ID.with(|value| *value.borrow_mut() = 0);
    SYNCED_LATEST_ROUND_ID.with(|value| *value.borrow_mut() = 0);
}

#[query]
fn debug_latest_round_id() -> u128 {
    get_latest_round_id()
}
#[query]
fn debug_round(idx: u128) -> Round {
    get_round(idx)
}
#[query]
fn debug_rounds_length() -> u128 {
    ROUNDS.with(|rounds| rounds.borrow().len()) as u128
}
#[query]
fn debug_synced_latest_round_id() -> u128 {
    get_synced_latest_round_id()
}
#[update]
fn debug_periodic_sync_state(
    interval_secs: u64,
    max_run_unit: u128,
    gas_coefficient_molecule: Option<u128>,
    gas_coefficient_denominator: Option<u128>,
    gas_limit: Option<u128>
) {
    periodic_sync_state_internal(
        interval_secs,
        max_run_unit,
        gas_coefficient_molecule,
        gas_coefficient_denominator,
        gas_limit
    )
}
#[update]
fn debug_stop_timer() {
    let timer_id = TIMER_ID.with(|value| value.borrow().clone());
    ic_cdk_timers::clear_timer(timer_id);
}
#[query]
fn debug_get_oracle_addr() -> String {
    oracle_addr()
}
#[update]
fn debug_update_oracle_addr(addr: String) -> String {
    ORACLE_ADDR.with(|val| {
        let mut val_mut = val.borrow_mut();
        *val_mut = addr;
        val_mut.clone()
    })
}
