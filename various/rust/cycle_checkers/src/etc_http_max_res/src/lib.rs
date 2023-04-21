use ic_cdk::{api::management_canister::http_request::{http_request, HttpHeader, CanisterHttpRequestArgument, HttpMethod, TransformArgs, HttpResponse, TransformContext}, query};
use ic_cdk_macros::update;
use serde_json::{json, Value};

#[query]
fn transform(response: TransformArgs) -> HttpResponse {
    let res = response.response;
    HttpResponse { status: res.status, headers: Vec::default(), body: res.body }
}


#[update]
async fn call_eth_block_number(max_response_bytes: u64) -> String {
    call_eth_block_number_internal(
        Some(max_response_bytes), 
        Some(TransformContext::new(transform, vec![]))
    ).await
}

async fn call_eth_block_number_internal(
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