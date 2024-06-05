use candid::{de, CandidType};
use serde::Deserialize;

#[derive(Debug)]
pub enum FunctionName {
    InitIn,
    Setup,
    SetTask,
}
impl From<&str> for FunctionName {
    fn from(s: &str) -> Self {
        match s {
            "init_in" => FunctionName::InitIn,
            "setup" => FunctionName::Setup,
            "set_task" => FunctionName::SetTask,
            _ => panic!("Invalid function name"),
        }
    }
}
impl FunctionName {
    pub fn to_string(&self) -> String {
        match self {
            FunctionName::InitIn => "init_in".to_string(),
            FunctionName::Setup => "setup".to_string(),
            FunctionName::SetTask => "set_task".to_string(),
        }
    }
}

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

#[derive(CandidType, Deserialize)]
pub struct Web3CtxParam {
    pub env: Env,
    pub url: String,
    pub from: Option<String>,
    pub chain_id: u64,
}
