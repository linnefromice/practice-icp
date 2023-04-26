use ic_web3::{
    ic::{get_public_key, pubkey_to_address},
    transports::ICHttp,
    types::Address,
    Web3,
};

use crate::{key_name, rpc_url};

pub fn default_derivation_key() -> Vec<u8> {
    ic_cdk::id().as_slice().to_vec()
}

pub fn generate_web3_client(
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<Web3<ICHttp>, String> {
    match ICHttp::new(rpc_url().as_str(), max_resp, cycles) {
        Ok(v) => Ok(Web3::new(v)),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn public_key() -> Result<Vec<u8>, String> {
    get_public_key(None, vec![default_derivation_key()], key_name().to_string()).await
}

pub fn to_ethereum_address(pub_key: Vec<u8>) -> Result<Address, String> {
    pubkey_to_address(&pub_key)
}

pub async fn ethereum_address() -> Result<Address, String> {
    let pub_key = public_key().await?;
    to_ethereum_address(pub_key)
}
