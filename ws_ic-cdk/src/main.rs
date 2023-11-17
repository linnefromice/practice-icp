use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument as CanisterHttpRequestArgumentV11;
use ic_cdk_v08::api::management_canister::http_request::CanisterHttpRequestArgument as CanisterHttpRequestArgumentV08;

fn main() {
    println!("Hello, world!");
}

fn http_request_required_cycles_v08(arg: &CanisterHttpRequestArgumentV08) -> u128 {
    let max_response_bytes = match arg.max_response_bytes {
        Some(ref n) => *n as u128,
        None => 2 * 1024 * 1024u128, // default 2MiB
    };
    let arg_raw = ic_cdk_v08::export::candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");
    // The coefficients can be found in [this page](https://internetcomputer.org/docs/current/developer-docs/production/computation-and-storage-costs).
    // 12 is "http_request".len().
    400_000_000u128 + 100_000u128 * (arg_raw.len() as u128 + 12 + max_response_bytes)
}

fn http_request_required_cycles_v11(arg: &CanisterHttpRequestArgumentV11) -> u128 {
    let max_response_bytes = match arg.max_response_bytes {
        Some(ref n) => *n as u128,
        None => 2 * 1024 * 1024u128, // default 2MiB
    };
    let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");
    // The fee is for a 13-node subnet to demonstrate a typical usage.
    (3_000_000u128
        + 60_000u128 * 13
        + (arg_raw.len() as u128 + "http_request".len() as u128) * 400
        + max_response_bytes * 800)
        * 13
}

#[cfg(test)]
mod tests_v08 {
    use super::*;

    fn arg() -> CanisterHttpRequestArgumentV08 {
        CanisterHttpRequestArgumentV08 {
            url: "https://example.com".to_string(),
            max_response_bytes: Some(3000),
            method: ic_cdk_v08::api::management_canister::http_request::HttpMethod::GET,
            headers: vec![],
            body: None,
            transform: None,
        }
    }

    #[test]
    fn required_cycles_with_some_max() {
        assert_eq!(http_request_required_cycles_v08(&arg()), 718_500_000);
    }

    #[test]
    fn required_cycles_with_none_max() {
        let mut arg = arg();
        arg.max_response_bytes = None;
        assert_eq!(http_request_required_cycles_v08(&arg), 210_132_900_000);
    }
}

#[cfg(test)]
mod tests_v11 {
    use super::*;

    fn arg() -> CanisterHttpRequestArgumentV11 {
        CanisterHttpRequestArgumentV11 {
            url: "https://example.com".to_string(),
            max_response_bytes: Some(3000),
            method: ic_cdk::api::management_canister::http_request::HttpMethod::GET,
            headers: vec![],
            body: None,
            transform: None,
        }
    }

    #[test]
    fn required_cycles_with_some_max() {
        assert_eq!(http_request_required_cycles_v11(&arg()), 81_302_000);
    }

    #[test]
    fn required_cycles_with_none_max() {
        let mut arg = arg();
        arg.max_response_bytes = None;
        assert_eq!(http_request_required_cycles_v11(&arg), 21_860_441_200);
    }
}