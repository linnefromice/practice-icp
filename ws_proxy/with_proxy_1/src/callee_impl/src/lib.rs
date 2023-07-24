use candid::{Decode, Encode};
use chainsight_cdk_macros::{did_export, init_in, prepare_stable_structure, stable_memory_for_vec, StableMemoryStorable};

init_in!();
prepare_stable_structure!();
stable_memory_for_vec!("snapshot", Snapshot, 0, true);
#[derive(
    Debug,
    Clone,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    StableMemoryStorable,
)]
#[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)]
pub struct Snapshot(pub String);

#[candid::candid_method(query)]
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}! I'm caller.", name)
}

#[candid::candid_method(update)]
#[ic_cdk::update]
fn insert_snapshot(value: Snapshot) -> Result<(), String> {
    add_snapshot(value)
}

did_export!("callee_impl");
