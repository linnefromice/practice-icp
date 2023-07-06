use std::{borrow::Cow, cell::RefCell};

use candid::{Decode, Encode};
use chainsight_cdk_macros::did_export;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BoundedStorable, DefaultMemoryImpl, Storable,
};
use ic_stable_structures::{StableCell, StableVec};

type CustomType = (
    String,
    i32,
    // u16,
    // u16,
    // u16,
    // u16,
    bool,
);

#[derive(Clone, Default, candid::CandidType, candid::Deserialize)]
pub struct SnapshotTuple(CustomType);
impl Storable for SnapshotTuple {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(candid::Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        candid::Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for SnapshotTuple {
    const MAX_SIZE: u32 = 100;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Clone, Default, candid::CandidType, candid::Deserialize)]
pub struct SnapshotStruct {
    pub value: CustomType,
    pub timestamp: u64,
}
impl Storable for SnapshotStruct {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(candid::Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        candid::Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for SnapshotStruct {
    const MAX_SIZE: u32 = 100;
    const IS_FIXED_SIZE: bool = false;
}

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<ic_stable_structures::memory_manager::MemoryManager<ic_stable_structures::DefaultMemoryImpl>>   =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static TUPLE_CELL: RefCell<StableCell<SnapshotTuple, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(0))),
            SnapshotTuple::default()
        ).unwrap()
    );

    static TUPLE_VEC: RefCell<StableVec<SnapshotTuple,Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(1)))
        ).unwrap()
    );

    static STRUCT_CELL: RefCell<StableCell<SnapshotStruct, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(2))),
            SnapshotStruct::default()
        ).unwrap()
    );

    static STRUCT_VEC: RefCell<StableVec<SnapshotStruct,Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(3)))
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
fn get_tuple_cell() -> SnapshotTuple {
    TUPLE_CELL.with(|cell| cell.borrow().get().clone())
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn set_tuple_cell(val: SnapshotTuple) -> Result<(), String> {
    let res = TUPLE_CELL.with(|cell| cell.borrow_mut().set(val));
    res.map(|_| ()).map_err(|e| format!("{:?}", e))
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_tuple_vec(idx: u64) -> Option<SnapshotTuple> {
    TUPLE_VEC.with(|vec| vec.borrow().get(idx))
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn insert_to_tuple_vec(val: SnapshotTuple) -> Result<(), String> {
    let res = TUPLE_VEC.with(|vec| vec.borrow_mut().push(&val));
    res.map(|_| ()).map_err(|e| format!("{:?}", e))
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_struct_cell() -> SnapshotStruct {
    STRUCT_CELL.with(|cell| cell.borrow().get().clone())
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn set_struct_cell(val: SnapshotStruct) -> Result<(), String> {
    let res = STRUCT_CELL.with(|cell| cell.borrow_mut().set(val));
    res.map(|_| ()).map_err(|e| format!("{:?}", e))
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_struct_vec(idx: u64) -> Option<SnapshotStruct> {
    STRUCT_VEC.with(|vec| vec.borrow().get(idx))
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn insert_to_struct_vec(val: SnapshotStruct) -> Result<(), String> {
    let res = STRUCT_VEC.with(|vec| vec.borrow_mut().push(&val));
    res.map(|_| ()).map_err(|e| format!("{:?}", e))
}

did_export!("custom_example");
