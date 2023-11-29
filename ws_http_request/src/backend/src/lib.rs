mod types;
use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext, TransformArgs, HttpResponse};
use types::SnapshotValue;

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
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
async fn call_spx() -> Result<SnapshotValue> {
    call_internal_struct::<SnapshotValue>("https://query2.finance.yahoo.com/v8/finance/chart/%5ESPX".to_string()).await
}

#[allow(non_snake_case)]
#[ic_cdk::update]
#[candid::candid_method(update)]
async fn call_SPX240315C04500000() -> Result<SnapshotValue> {
    call_internal_struct::<SnapshotValue>("https://query2.finance.yahoo.com/v8/finance/chart/SPX240315C04500000".to_string()).await
}

#[allow(non_snake_case)]
#[ic_cdk::update]
#[candid::candid_method(update)]
async fn call_SPX240315P04500000() -> Result<SnapshotValue> {
    call_internal_struct::<SnapshotValue>("https://query2.finance.yahoo.com/v8/finance/chart/SPX240315P04500000".to_string()).await
}

type Result<V> = std::result::Result<V, String>;
async fn call_internal_struct<V>(url: String) -> Result<V>
where
    V: serde::de::DeserializeOwned,
{
    let args = http_request_args(url);

    match http_request(args).await {
        Ok((res,)) => {
            let res: V = serde_json::from_slice(&res.body).expect("body is not valid json");
            Ok(res)
        },
        Err((r, m)) => Err(format!("Error: {:?} {:?}", r, m)),
    }
}

async fn call_internal_str(url: String) -> String {
    let args = http_request_args(url);

    match http_request(args).await {
        Ok((res,)) => {
            let body_str = String::from_utf8(res.body);
            ic_cdk::println!("{:?}", &body_str);
            body_str.expect("body is not valid utf8")
        },
        Err((r, m)) => format!("Error: {:?} {:?}", r, m),
    }
}

fn http_request_args(url: String) -> CanisterHttpRequestArgument {
    let headers: Vec<HttpHeader> = vec![
        HttpHeader { name :"Content-Type".to_string(),value:"application/json".to_string() },
    ];
    CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        headers,
        max_response_bytes: None,
        transform: Some(TransformContext::from_name("transform".to_string(), vec![])),
        body: None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }
}