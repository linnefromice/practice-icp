use candid::Principal;
use ic_cdk_macros::{query, update};
use ic_web3::{types::Address, ic::{get_public_key, pubkey_to_address}};

const KEY_NAME: &str = "dfx_test_key";

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
async fn eth_addr() -> String {
    let res = get_eth_addr(None, None, KEY_NAME.to_string()).await;
    match res {
        Ok(addr) => hex::encode(addr),
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
        Ok(pubkey) => { return pubkey_to_address(&pubkey); },
        Err(e) => { return Err(e); },
    };
}
