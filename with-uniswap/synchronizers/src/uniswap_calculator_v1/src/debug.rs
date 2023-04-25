use ic_cdk::{api::call, update};

use crate::READER_CANISTERS;

#[update]
async fn debug_rpcs() -> Vec<(String, String)> {
    let canisters = READER_CANISTERS.with(|value| value.borrow().clone());
    let mut results = Vec::<(String, String)>::new();
    for canister in canisters {
        let result = call::call::<_, (String,)>(canister, "debug_get_rpc_url", ()).await;
        results.push((canister.to_string(), result.unwrap().0));
    }
    results
}
