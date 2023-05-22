mod store;

use candid::Principal;
use ic_web3_macros::cross_canister_call_func;

type CallCanisterArgs = (String, String);
type CallCanisterResponse = Result<String, String>;
cross_canister_call_func!("greet", CallCanisterArgs, CallCanisterResponse);

#[ic_cdk::update]
async fn call(canister_id: String, name: String, msg: String) -> Result<String, String> {
    ic_cdk::println!("calling");
    let canister_id = Principal::from_text(canister_id).unwrap();
    call_greet(canister_id, (name, msg)).await
}

#[ic_cdk::query]
fn greet(name: String, msg: String) -> Result<String, String> {
    ic_cdk::println!("called");
    Ok(format!("Hello, {}! {}", name, msg))
}
