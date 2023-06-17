use std::str::FromStr;

use ic_cdk::api::management_canister::http_request::{TransformArgs, HttpResponse};
use ic_web3_rs::types::Address;
use ic_solidity_bindgen::Web3Context;

ic_solidity_bindgen::contract_abi!("./__interfaces/UniswapV3Pool.json");

const MAINNET_RPC_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy";
const POOL_ADDRESS: &str = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"; // USDC/ETH/500 pool

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
async fn token0() -> String {
    let addr = UniswapV3Pool::new(
        Address::from_str(POOL_ADDRESS).unwrap(),
        &web3_ctx(),
    ).token_0(None).await.unwrap();
    hex::encode(addr)
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn token1() -> String {
    let addr = UniswapV3Pool::new(
        Address::from_str(POOL_ADDRESS).unwrap(),
        &web3_ctx(),
    ).token_1(None).await.unwrap();
    hex::encode(addr)
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn fee() -> u32 {
    UniswapV3Pool::new(
        Address::from_str(POOL_ADDRESS).unwrap(),
        &web3_ctx(),
    ).fee(None).await.unwrap()
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn slot0() -> (String, i32, u16) {
    let res = UniswapV3Pool::new(
        Address::from_str(POOL_ADDRESS).unwrap(),
        &web3_ctx(),
    ).slot_0(None).await.unwrap();
    (
        res.0.to_string(),
        res.1,
        res.2
    )
}

chainsight_cdk_macros::did_export!("http_outcall_pool");
