use ic_cdk::{query, api::management_canister::http_request::{TransformArgs, HttpResponse}};
use ic_cdk_macros::update;
use ic_web3::{transports::ICHttp, Web3};

#[query]
fn transform(response: TransformArgs) -> HttpResponse {
    let res = response.response;
    HttpResponse { status: res.status, headers: Vec::default(), body: res.body }
}

#[update]
async fn chain_id() -> Result<String, String> {
    let transport = ICHttp::new(
        "https://polygon-mumbai.g.alchemy.com/v2/6GLIzI5pL0n4bp4c3jESZTRfXxE5XJ_Z",
        None,
        None
    ).unwrap();
    let gas_price = Web3::new(transport).eth()
        .chain_id()
        .await
        .map_err(|e| format!("get gas_price error: {}", e))?;
    Ok(gas_price.to_string())
}

#[update]
async fn block_number() -> Result<String, String> {
    let transport = ICHttp::new(
        "https://polygon-mumbai.g.alchemy.com/v2/6GLIzI5pL0n4bp4c3jESZTRfXxE5XJ_Z",
        None,
        None
    ).unwrap();
    let gas_price = Web3::new(transport).eth()
        .block_number()
        .await
        .map_err(|e| format!("get gas_price error: {}", e))?;
    Ok(gas_price.to_string())
}

#[update]
async fn gas_price() -> Result<String, String> {
    let transport = ICHttp::new(
        "https://polygon-mumbai.g.alchemy.com/v2/6GLIzI5pL0n4bp4c3jESZTRfXxE5XJ_Z",
        None,
        None
    ).unwrap();
    let gas_price = Web3::new(transport).eth()
        .gas_price()
        .await
        .map_err(|e| format!("get gas_price error: {}", e))?;
    Ok(gas_price.to_string())
}
