use std::cell::RefCell;

use chainsight_cdk_macros::did_export;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};
use ic_stable_structures::{StableCell, StableBTreeMap, StableVec};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static CELL: RefCell<StableCell<u128, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(0))),
            Default::default()
        ).unwrap()
    );

    static VEC: RefCell<StableVec<u128, Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(1))),
        ).unwrap()
    );

    static MAP: RefCell<StableBTreeMap<u128, u128, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(2))),
        )
    );
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

//// Cell
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_cell() -> u128 {
    CELL.with(|cell| cell.borrow().get().clone())
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn set_cell(val: u128) -> Result<(), String> {
    let res = CELL.with(|cell| cell.borrow_mut().set(val));
    res.map(|_| ()).map_err(|e| format!("{:?}", e))
}

//// Vec
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_vec_val(idx: u64) -> Option<u128> {
    VEC.with(|mem| mem.borrow().get(idx))
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_vec_val_unwrap(idx: u64) -> u128 {
    VEC.with(|mem| mem.borrow().get(idx)).unwrap()
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_vec_vals_len() -> u64 {
    VEC.with(|mem| mem.borrow().len())
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_last_vec_val() -> Option<u128> {
    VEC.with(|mem| {
        let borrowed_mem = mem.borrow();
        let len = borrowed_mem.len();
        borrowed_mem.get(len - 1)
    })
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_last_vec_val_unwrap() -> u128 {
    VEC.with(|mem| {
        let borrowed_mem = mem.borrow();
        let len = borrowed_mem.len();
        borrowed_mem.get(len - 1)
    }).unwrap()
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_top_vec_vals(n: u64) -> Vec<u128> {
    VEC.with(|mem| {
        let borrowed_mem = mem.borrow();
        let start_index = borrowed_mem.len().saturating_sub(n);
        let mut vec_var: Vec<u128> = borrowed_mem.iter().collect();
        vec_var.split_off(start_index as usize).iter().rev().cloned().collect()
    })
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_top_vec_vals_v2(n: u64) -> Vec<u128> {
    VEC.with(|mem| {
        let borrowed_mem = mem.borrow();
        let len = borrowed_mem.len();
        let mut res = Vec::new();
        for i in 0..n {
            res.push(borrowed_mem.get(len - i - 1).unwrap());
        }
        res
    })
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_vec() -> Vec<u128> {
    VEC.with(|mem| mem.borrow().iter().collect())
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn add_vec_val(value: u128) -> Result<(), String> {
    let res = VEC.with(|vec| vec.borrow_mut().push(&value));
    res.map_err(|e| format!("{:?}", e))
}

//// Map
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

did_export!("basic_example");
