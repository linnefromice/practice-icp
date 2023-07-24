use candid::{Decode, Encode};
use chainsight_cdk_macros::did_export;

#[candid::candid_method(update)]
#[ic_cdk::update]
async fn call_to_impl(
    canister_id: candid::Principal,
    idx: u64,
) -> Result<String, String> {
    let res = ic_cdk::api::call::call::<_, (String,)>(
        canister_id,
        "get_snapshot",
        (idx,)
    ).await.map_err(|e| format!("call error: {:?}", e))?;

    Ok(res.0)
}

#[candid::candid_method(update)]
#[ic_cdk::update]
async fn call_to_impl_last(
    canister_id: candid::Principal,
) -> Result<String, String> {
    let res = ic_cdk::api::call::call::<_, (String,)>(
        canister_id,
        "get_last_snapshot",
        ()
    ).await.map_err(|e| format!("call error: {:?}", e))?;

    Ok(res.0)
}

did_export!("caller_impl");
