use candid::Principal;
use ic_web3_macros::cross_canister_call_func;

// async fn call_greet(
//     canister_id: Principal,
//     call_args: CallCanisterArgs,
// ) -> CallCanisterResponse {
//     let res = ic_cdk::api::call::call::<_, (CallCanisterResponse,)>(
//         canister_id,
//         "greet",
//         call_args
//     )
//         .await
//         .map_err(|e| format!("call error: {:?}", e))?;
//     res.0
// }

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
