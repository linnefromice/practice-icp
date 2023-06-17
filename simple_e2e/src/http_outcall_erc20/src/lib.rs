use std::str::FromStr;

use ic_cdk::api::management_canister::http_request::{TransformArgs, HttpResponse};
use ic_web3_rs::types::Address;
use ic_solidity_bindgen::Web3Context;

ic_solidity_bindgen::contract_abi!("./__interfaces/ERC20.json");

const MAINNET_RPC_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy";
const ERC20_ADDRESS: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F"; // DAI

fn web3_ctx() -> Web3Context {
    Web3Context::new(
        MAINNET_RPC_URL,
        Address::from_low_u64_be(0),
        1,
        "test_key_1".to_string(),
    )
    .unwrap()
}

#[ic_cdk::query(name = "transform")]
#[candid::candid_method(query, rename = "transform")]
fn transform(response: TransformArgs) -> HttpResponse {
    let res = response.response;
    // remove header
    HttpResponse {
        status: res.status,
        headers: Vec::default(),
        body: res.body,
    }
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn name() -> String {
    ERC20::new(
        Address::from_str(ERC20_ADDRESS).unwrap(),
        &web3_ctx(),
    ).name(None).await.unwrap()
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn symbol() -> String {
    ERC20::new(
        Address::from_str(ERC20_ADDRESS).unwrap(),
        &web3_ctx(),
    ).symbol(None).await.unwrap()
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn total_supply() -> String {
    ERC20::new(
        Address::from_str(ERC20_ADDRESS).unwrap(),
        &web3_ctx(),
    ).total_supply(None).await.unwrap().to_string()
}

chainsight_cdk_macros::did_export!("http_outcall_erc20");
