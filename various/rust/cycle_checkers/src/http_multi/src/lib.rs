use ic_cdk::{api::management_canister::http_request::{http_request, HttpHeader, CanisterHttpRequestArgument, HttpMethod, TransformArgs, HttpResponse}, query};
use ic_cdk_macros::update;
use ic_web3::{transports::ICHttp, Web3};
use serde_json::{json, Value};

#[query]
fn transform(response: TransformArgs) -> HttpResponse {
    response.response
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
async fn call_binance_api_no_res() -> () {
    call_binance_api().await;
}
#[update]
async fn call_binance_api_three() -> (String, String, String) {
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
    (
        match http_request(request.clone()).await {
            Ok((response,)) => {
                String::from_utf8(response.body)
                        .expect("Transformed response is not UTF-8 encoded.")
            },
            Err((_, m)) => {
                m
            }
        },
        match http_request(request.clone()).await {
            Ok((response,)) => {
                String::from_utf8(response.body)
                        .expect("Transformed response is not UTF-8 encoded.")
            },
            Err((_, m)) => {
                m
            }
        },
        match http_request(request.clone()).await {
            Ok((response,)) => {
                String::from_utf8(response.body)
                        .expect("Transformed response is not UTF-8 encoded.")
            },
            Err((_, m)) => {
                m
            }
        },
    )
}
#[update]
async fn call_binance_api_three_no_res() -> () {
    call_binance_api_three().await;
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
async fn call_randomuser_api_three() -> (String, String, String) {
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
    (
        match http_request(request.clone()).await {
            Ok((response,)) => {
                String::from_utf8(response.body)
                        .expect("Transformed response is not UTF-8 encoded.")
            },
            Err((_, m)) => {
                m
            }
        },
        match http_request(request.clone()).await {
            Ok((response,)) => {
                String::from_utf8(response.body)
                        .expect("Transformed response is not UTF-8 encoded.")
            },
            Err((_, m)) => {
                m
            }
        },
        match http_request(request.clone()).await {
            Ok((response,)) => {
                String::from_utf8(response.body)
                        .expect("Transformed response is not UTF-8 encoded.")
            },
            Err((_, m)) => {
                m
            }
        }
    )
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
#[update]
async fn call_eth_block_number_three() -> (String, String, String) {
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
    (
        match http_request(request.clone()).await {
            Ok((response,)) => {
                let json: Value = serde_json::from_slice(&response.body).expect("Transformed response is not JSON payload.");
                let result = json.get("result").unwrap().as_str().unwrap().to_owned();
                let block_number = u64::from_str_radix(&result[2..], 16).unwrap();
                block_number.to_string()
            },
            Err((_, m)) => {
                m
            }
        },
        match http_request(request.clone()).await {
            Ok((response,)) => {
                let json: Value = serde_json::from_slice(&response.body).expect("Transformed response is not JSON payload.");
                let result = json.get("result").unwrap().as_str().unwrap().to_owned();
                let block_number = u64::from_str_radix(&result[2..], 16).unwrap();
                block_number.to_string()
            },
            Err((_, m)) => {
                m
            }
        },
        match http_request(request.clone()).await {
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
    )
}

#[update]
async fn call_eth_block_number_with_icweb3() -> String {
    let transport = ICHttp::new(
        "https://polygon-mumbai.g.alchemy.com/v2/6GLIzI5pL0n4bp4c3jESZTRfXxE5XJ_Z",
        None,
        None
    ).unwrap();
    let web3 = Web3::new(transport);
    match web3.eth()
        .block_number()
        .await {
            Ok(block_number) => block_number.to_string(),
            Err(e) => e.to_string()
        }
}

#[update]
async fn call_eth_block_number_with_icweb3_three() -> (String, String, String) {
    let transport = ICHttp::new(
        "https://polygon-mumbai.g.alchemy.com/v2/6GLIzI5pL0n4bp4c3jESZTRfXxE5XJ_Z",
        None,
        None
    ).unwrap();
    let web3 = Web3::new(transport);
    (
        match web3.eth()
        .block_number()
        .await {
            Ok(block_number) => block_number.to_string(),
            Err(e) => e.to_string()
        },
        match web3.eth()
        .block_number()
        .await {
            Ok(block_number) => block_number.to_string(),
            Err(e) => e.to_string()
        },
        match web3.eth()
        .block_number()
        .await {
            Ok(block_number) => block_number.to_string(),
            Err(e) => e.to_string()
        }
    )
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