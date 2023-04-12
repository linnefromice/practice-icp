use candid::{Principal, CandidType};
use ic_cdk_macros::{query, update};
use ic_web3::{types::Address, ic::{get_public_key as get_public_key_internal, pubkey_to_address as pubkey_to_address_internal }};

const KEY_NAME: &str = "dfx_test_key";

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
    match get_public_key(None, vec![ic_cdk::id().as_slice().to_vec()], KEY_NAME.to_string()).await {
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
    let path = if let Some(v) = derivation_path { v } else { vec![ic_cdk::id().as_slice().to_vec()] };
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


