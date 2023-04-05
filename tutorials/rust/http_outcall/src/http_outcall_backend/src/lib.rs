use std::{cell::RefCell, future::Future};
use candid::types::number::Nat;
use ic_cdk::api::{management_canister::http_request::{HttpHeader, CanisterHttpRequestArgument, HttpMethod, HttpResponse, http_request}, call::CallResult};
use ic_cdk_macros::{query, update};

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
    call_binance_api().await
}

#[update]
async fn call_api() -> String {
    call_binance_api().await
}

async fn call_binance_api() -> String {
    let host = "www.binance.us";
    // let host = "randomuser.me";
    let mut host_header = host.clone().to_owned();
    host_header.push_str(":443");
    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: host_header,
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "http_outcall_backend_canister".to_string()
        }
    ];
    let url = format!("https://{host}/api/v3/ticker/price?symbol=ETHUSDT");
    // let url = format!("https://{host}/api?seed=seed&results=1");
    ic_cdk::api::print(url.clone());
    ic_cdk::api::print("Making IC http_request call now.");
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
