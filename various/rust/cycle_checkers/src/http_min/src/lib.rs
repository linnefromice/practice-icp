use std::cell::RefCell;
use ic_cdk::api::management_canister::http_request::{http_request, HttpHeader, CanisterHttpRequestArgument, HttpMethod};
use ic_cdk_macros::{query, update};
use serde_json::{json, Value};

thread_local! {
    static STATE_STR: RefCell<String> = RefCell::default();
}

#[query]
fn simple_query(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
fn simple_update(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
fn state_query() -> String {
    STATE_STR.with(|s| s.borrow().clone())
}

#[update]
fn state_update(name: String) -> () {
    STATE_STR.with(|s| *s.borrow_mut() = name.clone());
}

#[update]
async fn call_binance_api() -> String {
    let host = "www.binance.us";
    let request_headers = create_request_header(host);
    let url = format!("https://{host}/api/v3/ticker/price?symbol=ETHUSDT");
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };
    match http_request(request).await {
        Ok((response,)) => {
            String::from_utf8(response.body)
                    .expect("Transformed response is not UTF-8 encoded.")
        },
        Err((_, m)) => {
            m
        }
    }
}

#[update]
async fn call_randomuser_api() -> String {
    let host = "randomuser.me";
    let request_headers = create_request_header(host);
    let url = format!("https://{host}/api?seed=seed&results=1");
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };
    match http_request(request).await {
        Ok((response,)) => {
            String::from_utf8(response.body)
                    .expect("Transformed response is not UTF-8 encoded.")
        },
        Err((_, m)) => {
            m
        }
    }
}

#[update]
async fn call_eth_block_number() -> String {
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
        max_response_bytes: None,
        transform: None,
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