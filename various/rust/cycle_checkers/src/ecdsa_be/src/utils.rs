use candid::Principal;
use ic_web3::{types::Address, ic::{get_public_key as get_public_key_internal, pubkey_to_address as pubkey_to_address_internal }, transports::ICHttp, Web3};

pub const KN_IN_LOCAL: &str = "dfx_test_key";

const BASE_URL: &'static str = "polygon-mumbai.g.alchemy.com";
const PATH: &'static str = "/v2/6GLIzI5pL0n4bp4c3jESZTRfXxE5XJ_Z";
pub const CHAIN_ID: u64 = 80001;

pub fn get_rpc_endpoint() -> String {
    format!("https://{}{}", BASE_URL, PATH)
}

pub fn default_derivation_key() -> Vec<u8> {
    ic_cdk::id().as_slice().to_vec()
}

pub fn generate_web3_client(max_resp: Option<u64>, cycles: Option<u64>) -> Result<Web3<ICHttp>, String> {
    match ICHttp::new(
        get_rpc_endpoint().as_str(),
        max_resp,
        cycles
    ) {
        Ok(v) => Ok(Web3::new(v)),
        Err(e) => Err(e.to_string())
    }
}

pub async fn get_public_key(
    canister_id: Option<Principal>,
    derivation_path: Vec<Vec<u8>>,
    name: String
) -> Result<Vec<u8>, String> {
    get_public_key_internal(canister_id, derivation_path, name).await
}

pub fn pubkey_to_address(pubkey: &[u8]) -> Result<Address, String> {
    pubkey_to_address_internal(&pubkey)
}