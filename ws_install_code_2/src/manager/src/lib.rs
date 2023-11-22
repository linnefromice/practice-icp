use candid::Principal;
use ic_cdk::api::{management_canister::main::{install_code, CanisterInstallMode, InstallCodeArgument}, call::CallResult};

const BACKEND_WASM: &[u8] = include_bytes!("../../../.dfx/local/canisters/backend_by_canister/backend_by_canister.wasm");

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn upgrade_backend(target: Principal) -> CallResult<()> {
    install(
        target,
        BACKEND_WASM.to_vec(),
        Vec::new()
    ).await
}

async fn install(canister_id: Principal, wasm_module: Vec<u8>, arg: Vec<u8>) -> CallResult<()> {
    install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Upgrade,
        canister_id,
        wasm_module,
        arg,
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