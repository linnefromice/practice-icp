use std::cell::RefCell;

use chainsight_cdk_macros::did_export;
use ic_stable_structures::{StableBTreeMap, StableVec};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static MAP: RefCell<StableBTreeMap<u128, u128, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(0))),
        )
    );

    static VEC: RefCell<StableVec<u128, Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(1))),
        ).unwrap()
    );
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_map(key: u128) -> Option<u128> {
    MAP.with(|map| map.borrow().get(&key))
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn set_map(key: u128, value: u128) {
    MAP.with(|map| map.borrow_mut().insert(key, value));
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_vec(idx: u64) -> Option<u128> {
    VEC.with(|vec| vec.borrow().get(idx as u64))
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn insert_to_vec(value: u128) -> Result<(), String> {
    let res = VEC.with(|vec| vec.borrow_mut().push(&value));
    res.map_err(|e| format!("{:?}", e))
}

did_export!("basic_example");
