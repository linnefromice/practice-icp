use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub enum Env {
    Production,
    Test,
    LocalDevelopment,
}

#[derive(Default, CandidType, Deserialize)]
pub struct CycleManagement {
    pub refueling_amount: candid::Nat,
    pub initial_supply: candid::Nat,
    pub refueling_threshold: candid::Nat,
}

#[derive(Default, CandidType, Deserialize)]
pub struct CycleManagements {
    pub db: CycleManagement,
    pub vault_intial_supply: candid::Nat,
    pub refueling_interval: u64,
    pub proxy: CycleManagement,
    pub indexer: CycleManagement,
}
