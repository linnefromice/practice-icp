use hex::FromHex;
use std::cell::RefCell;
use candid::types::number::Nat;
use ic_cdk::api::management_canister::http_request::{HttpHeader, CanisterHttpRequestArgument, HttpMethod, http_request};
use ic_cdk_macros::{query, update};
use serde_json::{json, Value};
use futures::join;

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0))
}

#[query]
fn get() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

#[update]
fn set(n: Nat) {
    COUNTER.with(|count| *count.borrow_mut() = n);
}

#[update]
fn inc() {
    COUNTER.with(|count| *count.borrow_mut() += 1);
}

#[query]
async fn call_api_query() -> String {
    call_binance_api_internal().await
}

#[update]
async fn call_binance_api() -> String {
    call_binance_api_internal().await
}

#[update]
async fn call_randomuser_api() -> String {
    call_randomuser_api_internal().await
}

#[update]
async fn call_eth_block_number() -> String {
    call_eth_block_number_internal().await
}

#[update]
async fn call_eth_call(to: String, data: String) -> String {
    let res = call_eth_call_internal(to.as_str(), data.as_str()).await;
    return match res {
        Ok(value) => value.get("result").unwrap().as_str().unwrap().to_string(),
        Err(msg) => msg
    }
}

#[update]
async fn call_eth_call_name(to: String) -> String {
    let data = "0x06fdde03"; // name()
    let res = call_eth_call_internal(to.as_str(), data).await;
    return match res {
        Ok(value) => {
            let result = value.get("result").unwrap().as_str().unwrap();
            let bytes = Vec::from_hex(&result[2..]).unwrap();
            String::from_utf8(bytes).unwrap()
        },
        Err(msg) => msg
    }
}

#[update]
async fn call_eth_call_symbol(to: String) -> String {
    let data = "0x95d89b41"; // symbol()
    let res = call_eth_call_internal(to.as_str(), data).await;
    return match res {
        Ok(value) => {
            let result = value.get("result").unwrap().as_str().unwrap();
            let bytes = Vec::from_hex(&result[2..]).unwrap();
            String::from_utf8(bytes).unwrap()
        },
        Err(msg) => msg
    }
}

#[update]
async fn call_eth_call_decimals(to: String) -> u64 {
    let data = "0x313ce567"; // decimals()
    let res = call_eth_call_internal(to.as_str(), data).await;
    return match res {
        Ok(value) => {
            let result = value.get("result").unwrap().as_str().unwrap();
            u64::from_str_radix(&result[2..], 16).unwrap()
        },
        Err(_) => 0 // temp
    }
}

#[update]
async fn call_matic_token_metadata() -> (String, String, u64) {
    let to = "0x0000000000000000000000000000000000001010"; // USDC
    join!(
        call_eth_call_internal_for_string(
            to,
            "0x06fdde03" // name()
        ),
        call_eth_call_internal_for_string(
            to,
            "0x95d89b41" // symbol()
        ),
        call_eth_call_internal_for_nat64(
            to,
            "0x313ce567" // decimals()
        )
    )
}

#[update]
async fn call_usdc_token_metadata() -> (String, String, u64) {
    let to = "0x2791bca1f2de4661ed88a30c99a7a9449aa84174"; // USDC
    join!(
        call_eth_call_internal_for_string(
            to,
            "0x06fdde03" // name()
        ),
        call_eth_call_internal_for_string(
            to,
            "0x95d89b41" // symbol()
        ),
        call_eth_call_internal_for_nat64(
            to,
            "0x313ce567" // decimals()
        )
    )
}

async fn call_eth_call_internal_for_string(to: &str, data: &str) -> String {
    let res = call_eth_call_internal(to, data).await;
    return match res {
        Ok(value) => {
            let result = value.get("result").unwrap().as_str().unwrap();
            let bytes = Vec::from_hex(&result[2..]).unwrap();
            String::from_utf8(bytes).unwrap()
        },
        Err(msg) => msg
    }
}

async fn call_eth_call_internal_for_nat64(to: &str, data: &str) -> u64 {
    let res = call_eth_call_internal(to, data).await;
    return match res {
        Ok(value) => {
            let result = value.get("result").unwrap().as_str().unwrap();
            u64::from_str_radix(&result[2..], 16).unwrap()
        },
        Err(_) => 0 // temp
    }
}

async fn call_eth_block_number_internal() -> String {
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
            let result = json.get("result").unwrap().as_str().unwrap();
            let block_number = u64::from_str_radix(&result[2..], 16).unwrap();
            block_number.to_string()
        },
        Err((_, m)) => {
            m
        }
    }
}


async fn call_eth_call_internal(to: &str, data: &str) -> Result<Value, String> {
    let host = "polygon-mainnet.g.alchemy.com";
    let request_headers = create_request_header(host);

    let json_payload = json!({
        "jsonrpc": "2.0",
        "method": "eth_call",
        "id": 1,
        "params": [{
            "to": to,
            "data": data
        }, "latest"]
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
            Ok(serde_json::from_slice(&response.body).expect("Transformed response is not JSON payload."))
        },
        Err((_, m)) => {
            Err(m)
        }
    }
}

async fn call_binance_api_internal() -> String {
    let host = "www.binance.us";
    let request_headers = create_request_header(host);
    let url = format!("https://{host}/api/v3/ticker/price?symbol=ETHUSDT");
    ic_cdk::api::print(url.clone());
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

async fn call_randomuser_api_internal() -> String {
    let host = "randomuser.me";
    let request_headers = create_request_header(host);
    let url = format!("https://{host}/api?seed=seed&results=1");
    ic_cdk::api::print(url.clone());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set() {
        let expected = Nat::from(42);
        set(expected.clone());
        assert_eq!(get(), expected);
    }
    #[test]
    fn test_init() {
        assert_eq!(get(), Nat::from(0));
    }

    #[test]
    fn test_inc() {
        for i in 1..10 {
            inc();
            assert_eq!(get(), Nat::from(i));
        }
    }
}
