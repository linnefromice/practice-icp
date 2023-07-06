use std::{cell::RefCell, borrow::Cow};

use candid::{Encode, Decode};
use chainsight_cdk_macros::did_export;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, Storable, BoundedStorable,
};
use ic_stable_structures::{StableVec};

type Memory = VirtualMemory<DefaultMemoryImpl>;

type CustomType = (
    String,
    i32,
    bool,
);
#[derive(Clone, Default, candid::CandidType, candid::Deserialize)]
pub struct SnapshotTuple(CustomType);
impl Storable for SnapshotTuple {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for SnapshotTuple {
    const MAX_SIZE: u32 = 100;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static VEC: RefCell<StableVec<SnapshotTuple, Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(0))),
        ).unwrap()
    );
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

//// Vec
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_vec_val(idx: u64) -> (Option<SnapshotTuple>, u64) {
    let start = ic_cdk::api::instruction_counter();
    let res = VEC.with(|mem| mem.borrow().get(idx));
    (res, ic_cdk::api::instruction_counter() - start)
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_vec_vals_len() -> (u64, u64) {
    let start = ic_cdk::api::instruction_counter();
    let res = VEC.with(|mem| mem.borrow().len());
    (res, ic_cdk::api::instruction_counter() - start)
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_last_vec_val() -> (Option<SnapshotTuple>, u64) {
    let start = ic_cdk::api::instruction_counter();
    let res = VEC.with(|mem| {
        let borrowed_mem = mem.borrow();
        let len = borrowed_mem.len();
        borrowed_mem.get(len - 1)
    });
    (res, ic_cdk::api::instruction_counter() - start)
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_top_vec_vals(n: u64) -> (Vec<SnapshotTuple>, u64) {
    let start = ic_cdk::api::instruction_counter();
    let res = VEC.with(|mem| {
        let borrowed_mem = mem.borrow();
        let start_index = borrowed_mem.len().saturating_sub(n);
        let mut vec_var: Vec<SnapshotTuple> = borrowed_mem.iter().collect();
        vec_var.split_off(start_index as usize).iter().rev().cloned().collect()
    });
    (res, ic_cdk::api::instruction_counter() - start)
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_top_vec_vals_v2(n: u64) -> (Vec<SnapshotTuple>, u64) {
    let start = ic_cdk::api::instruction_counter();
    let res = VEC.with(|mem| {
        let borrowed_mem = mem.borrow();
        let len = borrowed_mem.len();
        let mut res = Vec::new();
        for i in 0..n {
            res.push(borrowed_mem.get(len - i - 1).unwrap());
        }
        res
    });
    (res, ic_cdk::api::instruction_counter() - start)
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_vec() -> (Vec<SnapshotTuple>, u64) {
    let start = ic_cdk::api::instruction_counter();
    let res = VEC.with(|mem| mem.borrow().iter().collect());
    (res, ic_cdk::api::instruction_counter() - start)
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn add_vec_val(value: SnapshotTuple) -> (Result<(), String>, u64) {
    let start = ic_cdk::api::instruction_counter();
    let res = VEC.with(|vec| vec.borrow_mut().push(&value));
    (
        res.map_err(|e| format!("{:?}", e)),
        ic_cdk::api::instruction_counter() - start
    )
}

did_export!("basic_example_measurement");
