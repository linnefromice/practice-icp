use ic_cdk::{api::management_canister::http_request::{http_request, HttpHeader, CanisterHttpRequestArgument, HttpMethod, TransformArgs, HttpResponse, TransformContext}, query};
use ic_cdk_macros::update;
use ic_web3::{transports::ICHttp, Web3};
use serde_json::{json, Value};

#[query]
fn transform(response: TransformArgs) -> HttpResponse {
    response.response
}

#[query]
fn transform_no_header(response: TransformArgs) -> HttpResponse {
    let res = response.response;
    HttpResponse { status: res.status, headers: Vec::default(), body: res.body }
}


#[update]
async fn call_default() -> String {
    call_eth_block_number(None, None).await
}
#[update]
async fn call_same_icweb3_default() -> String {
    call_eth_block_number(
        Some(500_000), 
        Some(TransformContext::new(transform, vec![]))
    ).await
}
#[update]
async fn call_max_50000_default() -> String {
    call_eth_block_number(
        Some(50_000),
        None
    ).await
}
#[update]
async fn call_max_25000_default() -> String {
    call_eth_block_number(
        Some(25_000), 
        None
    ).await
}
#[update]
async fn call_max_5000_default() -> String {
    call_eth_block_number(
        Some(5_000), 
        None
    ).await
}
#[update]
async fn call_without_header() -> String {
    call_eth_block_number(
        None,
        Some(TransformContext::new(transform_no_header, vec![]))
    ).await
}
#[update]
async fn call_without_header_max_50000() -> String {
    call_eth_block_number(
        Some(50_000),
        Some(TransformContext::new(transform_no_header, vec![]))
    ).await
}
#[update]
async fn call_without_header_max_25000() -> String {
    call_eth_block_number(
        Some(25_000),
        Some(TransformContext::new(transform_no_header, vec![]))
    ).await
}
#[update]
async fn call_without_header_max_5000() -> String {
    call_eth_block_number(
        Some(5_000),
        Some(TransformContext::new(transform_no_header, vec![]))
    ).await
}

async fn call_eth_block_number(
    max_response_bytes: Option<u64>,
    transform: Option<TransformContext>
) -> String {
    let host = "polygon-mainnet.g.alchemy.com";
    let request_headers = create_request_header(host);

    let json_payload = json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "id": 1
    });
    let body = serde_json::to_vec(&json_payload).unwrap();
    let url = format!("https://{host}/v2/sLp6VfuskMEwx8Wx0DvaRkI8qCoVYF8f");
        let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::POST,
        body: Some(body),
        max_response_bytes: max_response_bytes,
        transform: transform,
        headers: request_headers,
    };
    match http_request(request).await {
        Ok((response,)) => {
            let json: Value = serde_json::from_slice(&response.body).expect("Transformed response is not JSON payload.");
            let result = json.get("result").unwrap().as_str().unwrap().to_owned();
            let block_number = u64::from_str_radix(&result[2..], 16).unwrap();
            block_number.to_string()
        },
        Err((_, m)) => {
            m
        }
    }
}

#[update]
async fn call_default_with_icweb3() -> String {
    call_eth_block_number_with_icweb3(None).await
}
#[update]
async fn call_max_50000_with_icweb3() -> String {
    call_eth_block_number_with_icweb3(Some(50_000)).await
}
#[update]
async fn call_max_25000_with_icweb3() -> String {
    call_eth_block_number_with_icweb3(Some(25_000)).await
}
#[update]
async fn call_max_5000_with_icweb3() -> String {
    call_eth_block_number_with_icweb3(Some(5_000)).await
}

async fn call_eth_block_number_with_icweb3(
    max_response_bytes: Option<u64>,
) -> String {
    let transport = ICHttp::new(
        "https://polygon-mainnet.g.alchemy.com/v2/sLp6VfuskMEwx8Wx0DvaRkI8qCoVYF8f",
        max_response_bytes,
        None,
    ).unwrap();
    let web3 = Web3::new(transport);
    match web3.eth()
        .block_number()
        .await {
            Ok(block_number) => block_number.to_string(),
            Err(e) => e.to_string()
        }
}

fn create_request_header(host: &str) -> Vec<HttpHeader> {
    let mut host_header = host.clone().to_owned();
    host_header.push_str(":443");
    vec![
        HttpHeader {
            name: "Host".to_string(),
            value: host_header,
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "http_outcall_backend_canister".to_string()
        }
    ]
}