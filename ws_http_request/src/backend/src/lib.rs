use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext, TransformArgs, HttpResponse};

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
async fn call() -> String {
    let headers: Vec<HttpHeader> = vec![
        HttpHeader { name :"Content-Type".to_string(),value:"application/json".to_string() },
    ];
    let args = CanisterHttpRequestArgument {
        url: "https://query2.finance.yahoo.com/v8/finance/chart/%5ESPX".to_string(),
        method: HttpMethod::GET,
        headers,
        max_response_bytes: None,
        transform: Some(TransformContext::from_name("transform".to_string(), vec![])),
        body: None,
    };

    match http_request(args).await {
        Ok((res,)) => {
            let body_str = String::from_utf8(res.body);
            ic_cdk::println!("{:?}", &body_str);
            body_str.expect("body is not valid utf8")
        },
        Err((r, m)) => format!("Error: {:?} {:?}", r, m),
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