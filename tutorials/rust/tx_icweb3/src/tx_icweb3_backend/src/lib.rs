use std::str::FromStr;

use candid::{Principal, CandidType};
use ic_cdk_macros::{query, update};
use ic_web3::{types::{Address, SignedTransaction, TransactionParameters, U256}, ic::{get_public_key as get_public_key_internal, pubkey_to_address as pubkey_to_address_internal, KeyInfo }, transports::ICHttp, Web3};

const KEY_NAME: &str = "dfx_test_key";

const BASE_URL: &'static str = "polygon-mainnet.g.alchemy.com";
const PATH: &'static str = "/v2/sLp6VfuskMEwx8Wx0DvaRkI8qCoVYF8f";
const CHAIN_ID: u64 = 1;

fn get_rpc_endpoint() -> String {
    format!("https://{}{}", BASE_URL, PATH)
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(CandidType)]
struct AccountInfo {
    pub address: String,
    pub pub_key: String
}

#[update]
async fn account_info() -> Result<AccountInfo, String> {
    let pub_key = get_public_key(None, vec![ic_cdk::id().as_slice().to_vec()], KEY_NAME.to_string()).await;
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
async fn pub_key() -> String {
    match get_public_key(None, vec![default_derivation_key()], KEY_NAME.to_string()).await {
        Ok(v) => format!("0x{}", hex::encode(v)),
        Err(msg) => msg
    }
}

#[update]
async fn eth_addr() -> String {
    let res = get_eth_addr(None, None, KEY_NAME.to_string()).await;
    match res {
        Ok(v) => format!("0x{}", hex::encode(v)),
        Err(msg) => msg
    }
}

#[update]
async fn send_eth(to: String, value: u64) -> Result<String, String> {
    let res = send_eth_siged_tx(to, value).await;
    if let Err(msg) = res { return Err(msg) };
    let signed_tx = res.unwrap();
    ic_cdk::println!("message_hash: {:?}", signed_tx.message_hash.as_bytes());
    ic_cdk::println!("v: {:?}", signed_tx.v);
    ic_cdk::println!("r: {:?}", signed_tx.r);
    ic_cdk::println!("s: {:?}", signed_tx.s);
    ic_cdk::println!("raw_transaction: {:?}", signed_tx.raw_transaction);
    ic_cdk::println!("transaction_hash: {:?}", signed_tx.transaction_hash);
    Ok("OK".to_string())
}


async fn get_eth_addr(
    canister_id: Option<Principal>,
    derivation_path: Option<Vec<Vec<u8>>>,
    name: String
) -> Result<Address, String> {
    let path = if let Some(v) = derivation_path { v } else { vec![default_derivation_key()] };
    match get_public_key(canister_id, path, name).await {
        Ok(pubkey) => { return pubkey_to_address_internal(&pubkey); },
        Err(e) => { return Err(e); },
    };
}

async fn get_public_key(
    canister_id: Option<Principal>,
    derivation_path: Vec<Vec<u8>>,
    name: String
) -> Result<Vec<u8>, String> {
    get_public_key_internal(canister_id, derivation_path, name).await
}

fn pubkey_to_address(pubkey: &[u8]) -> Result<Address, String> {
    pubkey_to_address_internal(&pubkey)
}

fn default_derivation_key() -> Vec<u8> {
    ic_cdk::id().as_slice().to_vec()
}

async fn send_eth_siged_tx(to: String, value: u64) -> Result<SignedTransaction, String> {
    let derivation_path = vec![default_derivation_key()];
    let key_info = KeyInfo { derivation_path: derivation_path, key_name: KEY_NAME.to_string() };

    let from_addr = get_eth_addr(None, None, KEY_NAME.to_string())
        .await
        .map_err(|e| format!("get canister eth addr failed: {}", e));
    if let Err(msg) = from_addr {
        return Err(msg)
    }
    let w3 = match ICHttp::new(&get_rpc_endpoint(), None, None) {
        Ok(v) => { Web3::new(v) },
        Err(e) => { return Err(e.to_string()) },
    };

    let to = Address::from_str(&to).unwrap();
    let tx = TransactionParameters {
        to: Some(to),
        nonce: Some(U256::from(0)), // remember to fetch nonce first
        value: U256::from(value),
        gas_price: Some(U256::exp10(10)), // 10 gwei
        gas: U256::from(21000),
        ..Default::default()
    };
    let signed_tx = w3.accounts()
        .sign_transaction(tx, hex::encode(from_addr.unwrap()), key_info, CHAIN_ID)
        .await
        .map_err(|e| format!("sign tx error: {}", e));
    signed_tx
}