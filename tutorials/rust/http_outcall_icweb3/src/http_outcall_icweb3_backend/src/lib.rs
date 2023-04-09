use ic_cdk::{update, api::management_canister::http_request::{TransformArgs, HttpResponse}};
use ic_cdk_macros::query;
use ic_web3::{transports::ICHttp, Web3, types::{BlockId, BlockNumber, U64}};

// Constants
const BASE_URL: &'static str = "polygon-mainnet.g.alchemy.com";
const PATH: &'static str = "/v2/sLp6VfuskMEwx8Wx0DvaRkI8qCoVYF8f";

fn get_rpc_endpoint() -> String {
    format!("https://{}{}", BASE_URL, PATH)
}

async fn get_block(number: Option<u64>) -> Result<String, String> {
    let w3 = match ICHttp::new(get_rpc_endpoint().as_str(), None, None) {
        Ok(v) => Web3::new(v),
        Err(e) => return Err(e.to_string())
    };
    let block_id = match number {
        Some(id) => { BlockId::from(U64::from(id)) },
        None => { BlockId::Number(BlockNumber::Latest) },
    };
    let block = w3.eth().block(block_id).await.map_err(|e| format!("get block error: {}", e))?;

    Ok(serde_json::to_string(&block.unwrap()).unwrap())
}

#[query(name = "transform")]
fn transform(response: TransformArgs) -> HttpResponse {
    response.response
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
fn get_settings() -> String {
    return get_rpc_endpoint();
}

#[update]
async fn get_latest_block() -> String {
    match get_block(None).await {
        Ok(msg) => msg,
        Err(msg) => msg,
    }
}