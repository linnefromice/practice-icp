use ic_cdk::export::Principal;
use ic_cdk_timers::TimerId;
use std::cell::RefCell;

use crate::env::EcdsaKeyEnvs;

thread_local! {
    static HHI_CANISTER: RefCell<Principal> = RefCell::new(Principal::anonymous());
    static MAPPER: RefCell<String> = RefCell::new(String::default());
    static RPC_URL: RefCell<String>  = RefCell::default();
    static CHAIN_ID: RefCell<u64>  = RefCell::default();
    static ORACLE_ADDRESS: RefCell<String> = RefCell::default();
    static KEY_NAME: RefCell<String>  = RefCell::new(EcdsaKeyEnvs::LocalDevelopment.to_key_name());
    static TIMER_ID: RefCell<TimerId> = RefCell::default();
}

pub fn hhi_canister() -> Principal {
    HHI_CANISTER.with(|hhi_canister| *hhi_canister.borrow())
}
pub fn set_hhi_canister(hhi_canister_id: String) {
    HHI_CANISTER.with(|hhi_canister| {
        *hhi_canister.borrow_mut() = Principal::from_text(hhi_canister_id).unwrap();
    });
}

pub fn mapper() -> String {
    MAPPER.with(|mapper| mapper.borrow().clone())
}
pub fn set_mapper(mapper_canister_id: String) {
    MAPPER.with(|mapper| {
        *mapper.borrow_mut() = mapper_canister_id;
    });
}

pub fn rpc_url() -> String {
    RPC_URL.with(|rpc_url| rpc_url.borrow().clone())
}
pub fn set_rpc_url(rpc_url: String) {
    RPC_URL.with(|rpc_url_| {
        *rpc_url_.borrow_mut() = rpc_url;
    });
}

pub fn chain_id() -> u64 {
    CHAIN_ID.with(|chain_id| *chain_id.borrow())
}
pub fn set_chain_id(chain_id: u64) {
    CHAIN_ID.with(|chain_id_| {
        *chain_id_.borrow_mut() = chain_id;
    });
}

pub fn oracle_address() -> String {
    ORACLE_ADDRESS.with(|oracle_address| oracle_address.borrow().clone())
}
pub fn set_oracle_address(oracle_address: String) {
    ORACLE_ADDRESS.with(|oracle_address_| {
        *oracle_address_.borrow_mut() = oracle_address;
    });
}

pub fn key_name() -> String {
    KEY_NAME.with(|val| val.borrow().clone())
}
pub fn set_key_name(env: EcdsaKeyEnvs) {
    KEY_NAME.with(|value| {
        *value.borrow_mut() = env.to_key_name();
    });
}

pub fn timer_id() -> TimerId {
    TIMER_ID.with(|value| *value.borrow())
}
pub fn set_timer_id(timer_id: TimerId) {
    TIMER_ID.with(|value| {
        *value.borrow_mut() = timer_id;
    });
}
