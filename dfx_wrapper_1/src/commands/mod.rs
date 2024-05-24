use ic_agent::{agent::status::Status, export::Principal, Agent, AgentError};
use ic_utils::interfaces::ManagementCanister;

pub async fn ping(agent: &Agent) -> Result<Status, AgentError> {
    agent.status().await
}

// from sdk/src/dfx/src/lib/operations/canister/create_canister.rs
pub async fn canister_create(agent: &Agent) -> Result<Principal, AgentError> {
    let mgr = ManagementCanister::create(agent);
    let builder = mgr.create_canister();
    // .with_controller(controller) // attach controller
    let res = builder
        // .with_optional_compute_allocation(settings.compute_allocation)
        // .with_optional_memory_allocation(settings.memory_allocation)
        // .with_optional_freezing_threshold(settings.freezing_threshold)
        // .with_optional_reserved_cycles_limit(settings.reserved_cycles_limit)
        // .with_optional_wasm_memory_limit(settings.wasm_memory_limit)
        .call_and_wait()
        .await;
    match res {
        Ok((principal,)) => Ok(principal),
        Err(err) => Err(err),
    }
    // Err(UncertifiedReject(RejectResponse { reject_code: CanisterReject, reject_message: "ic00 method create_canister can be called only by a canister", error_code: Some("IC0516") }))
}
