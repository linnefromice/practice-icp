use std::str::FromStr;

use candid::{Principal, CandidType};
use ic_cdk_macros::{query, update};
use ic_web3::{types::{Address, SignedTransaction, TransactionParameters, U256}, ic::{get_public_key as get_public_key_internal, pubkey_to_address as pubkey_to_address_internal, KeyInfo, ic_raw_sign, recover_address }, transports::ICHttp, Web3, signing::hash_message};

const KEY_NAME: &str = "dfx_test_key";

const BASE_URL: &'static str = "polygon-mainnet.g.alchemy.com";
const PATH: &'static str = "/v2/sLp6VfuskMEwx8Wx0DvaRkI8qCoVYF8f";
const CHAIN_ID: u64 = 1;

#[derive(CandidType)]
struct AccountInfo {
    pub address: String,
    pub pub_key: String
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

fn get_rpc_endpoint() -> String {
    format!("https://{}{}", BASE_URL, PATH)
}

fn default_derivation_key() -> Vec<u8> {
    ic_cdk::id().as_slice().to_vec()
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
fn rpc_endpoint() -> String {
    get_rpc_endpoint()
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