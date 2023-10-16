use candid::{Decode, Encode};
use chainsight_cdk_macros::did_export;
use chainsight_cdk::rpc::{Message, Caller, CallProvider};

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

#[candid::candid_method(update)]
#[ic_cdk::update]
async fn call_to_proxy_last(
    proxy_id: candid::Principal,
    canister_id: candid::Principal,
) -> Result<String, String> {
    let call_result = CallProvider::new(proxy_id)
        .call(
            Message::new::<()>((), canister_id, "proxy_get_last_snapshot")
                .unwrap(),
        )
        .await;
    if let Err(err) = call_result {
        ic_cdk::println!("call_result error: {:?}", &err);
        return Err("call_result error".to_string());
    }
    let res = call_result.unwrap().reply::<String>();
    if let Err(err) = res {
        ic_cdk::println!("call_result.reply error: {:?}", err);
        return Err("call_result.reply error".to_string());
    }
    Ok(res.unwrap())
}


did_export!("caller_impl");
