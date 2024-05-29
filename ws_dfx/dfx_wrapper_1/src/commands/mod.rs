use dfx::{
    lib::{
        environment::EnvironmentImpl, ic_attributes::CanisterSettings,
        operations::canister::create_canister,
    },
    util::clap::subnet_selection_opt::SubnetSelectionType,
};
use dfx_core::identity::CallSender;
use ic_agent::{agent::status::Status, export::Principal, Agent, AgentError};
use slog::Logger;

pub async fn ping(agent: &Agent) -> Result<Status, AgentError> {
    agent.status().await
}

// from sdk/src/dfx/src/lib/operations/canister/create_canister.rs
pub async fn canister_create(logger: Logger) -> Result<(), anyhow::Error> {
    let env = generate_env(logger).unwrap();

    create_canister(
        &env,
        "backend",
        None,
        None,
        &CallSender::from(&None).unwrap(),
        false,
        None,
        CanisterSettings::default(),
        None,
        &mut SubnetSelectionType::Automatic {
            selected_subnet: None,
        },
    )
    .await
}

// pub async fn canister_create(logger: Logger, agent: &Agent) -> Result<Principal, anyhow::Error> {
// let mgr = ManagementCanister::create(agent);
// let builder = mgr.create_canister();
// // .with_controller(controller) // attach controller
// let res = builder
//     // .with_optional_compute_allocation(settings.compute_allocation)
//     // .with_optional_memory_allocation(settings.memory_allocation)
//     // .with_optional_freezing_threshold(settings.freezing_threshold)
//     // .with_optional_reserved_cycles_limit(settings.reserved_cycles_limit)
//     // .with_optional_wasm_memory_limit(settings.wasm_memory_limit)
//     .call_and_wait()
//     .await;
// match res {
//     Ok((principal,)) => Ok(principal),
//     Err(err) => Err(err),
// }
// Err(UncertifiedReject(RejectResponse { reject_code: CanisterReject, reject_message: "ic00 method create_canister can be called only by a canister", error_code: Some("IC0516") }))
// }

pub fn generate_env(logger: Logger) -> Result<EnvironmentImpl, anyhow::Error> {
    EnvironmentImpl::new().map(|env| {
        env.with_logger(logger)
            .with_identity_override(None)
            .with_verbose_level(-3)
            .with_effective_canister_id(None)
    })
}
