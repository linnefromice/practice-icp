use candid::Principal;
use ic_cdk::api::{management_canister::{main::{canister_status, CanisterStatusResponse}, provisional::CanisterIdRecord}, call::CallResult};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
async fn query_status_self() -> CanisterStatusResponse {
    call_canister_status(CanisterIdRecord { canister_id: ic_cdk::api::id() }).await.unwrap().0
}

#[ic_cdk::query]
#[candid::candid_method(query)]
async fn query_status(id: String) -> CanisterStatusResponse {
    let id = Principal::from_text(id).unwrap();
    call_canister_status(CanisterIdRecord { canister_id: id }).await.unwrap().0
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn call_status_self() -> CanisterStatusResponse {
    call_canister_status(CanisterIdRecord { canister_id: ic_cdk::api::id() }).await.unwrap().0
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn call_status(id: String) -> CanisterStatusResponse {
    let id = Principal::from_text(id).unwrap();
    call_canister_status(CanisterIdRecord { canister_id: id }).await.unwrap().0
}

async fn call_canister_status(id_record: CanisterIdRecord) -> CallResult<(CanisterStatusResponse,)> {
    canister_status(id_record)
        .await
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
