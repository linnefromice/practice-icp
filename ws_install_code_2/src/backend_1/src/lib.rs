use candid::Principal;
use ic_cdk::api::{call::CallResult, management_canister::{main::{UpdateSettingsArgument, update_settings}, provisional::CanisterSettings}};

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String, msg: String) -> String {
    format!("Hello, {}! Upgraded second! {}", name, msg)
}

#[ic_cdk::update]
#[candid::candid_method]
async fn add_controller(controller: Principal) -> CallResult<()> {
    let caller = ic_cdk::caller();
    let this_canister = ic_cdk::api::id();
    update_settings(UpdateSettingsArgument {
        canister_id: this_canister.clone(),
        settings: CanisterSettings {
            controllers: Some(vec![caller, this_canister, controller]), // NOTE: overwrite only
            compute_allocation: None,
            freezing_threshold: None,
            memory_allocation: None,
        },
    })
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