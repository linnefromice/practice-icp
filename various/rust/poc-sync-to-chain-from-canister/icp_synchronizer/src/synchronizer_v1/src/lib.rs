mod utils;

use std::str::FromStr;
use candid::CandidType;
use ic_cdk::{query, update, api::management_canister::http_request::{TransformArgs, HttpResponse}};
use ic_web3::{Web3, types::Address, transports::ICHttp, contract::{Contract, Options}};
use utils::{get_rpc_endpoint, KEY_NAME, default_derivation_key, get_public_key, pubkey_to_address, generate_web3_client};

// Oracle
const ORACLE_ADDR: &'static str = "af974dfd33cb1105710eddbb8f30f1ba3c994da1"; // remove 0x
const ORACLE_ABI: &[u8] = include_bytes!("../../abi/OracleV1.json");

#[derive(CandidType)]
struct AccountInfo {
    pub address: String,
    pub pub_key: String
}

#[query]
fn transform(response: TransformArgs) -> HttpResponse {
    response.response
}

fn generate_contract_client(w3: Web3<ICHttp>, contract_addr: &str, abi: &[u8]) -> Result<Contract<ICHttp>, String> {
    let contract_address = Address::from_str(contract_addr).unwrap();
    Contract::from_json(
        w3.eth(),
        contract_address,
        abi
    ).map_err(|e| format!("init contract failed: {}", e))
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