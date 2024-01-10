use std::cell::RefCell;

use candid::{Encode, Decode};
use ic_stable_structures::{DefaultMemoryImpl, memory_manager::{VirtualMemory, MemoryManager, MemoryId}};

mod types_all;
mod types_partial_1;
mod types_partial_2;

type MemoryType = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // heap
    static HEAP_DATUM: RefCell<Option<types_all::Snapshot>> = RefCell::new(None);

    // stable memory
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static STABLE_DATUM: RefCell<ic_stable_structures::Cell<types_all::Snapshot, MemoryType>> = RefCell::new(ic_stable_structures::Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), types_all::Snapshot::default()).unwrap());
    static STABLE_DATA: RefCell<ic_stable_structures::Vec<types_all::Snapshot, MemoryType>> = RefCell::new(ic_stable_structures::Vec::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))).unwrap());
    static STABLE_BYTES: RefCell<ic_stable_structures::Vec<types_all::SnapshotBytes, MemoryType>> = RefCell::new(ic_stable_structures::Vec::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))).unwrap());
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn all_dummy() -> types_all::Snapshot {
    types_all::dummy()
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn all_default() -> types_all::Snapshot {
    types_all::Snapshot::default()
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn all_dummy_bytes() -> (String, Vec<u8>) {
    let bytes = Encode!(&all_dummy()).unwrap();
    (
        format!("{:x?}", &bytes),
        bytes.clone()
    )
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn all_default_bytes() -> (String, Vec<u8>) {
    let bytes = Encode!(&all_default()).unwrap();
    (
        format!("{:x?}", &bytes),
        bytes.clone()
    )
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn all_dummy_re_bytes() -> (bool, String, String) {
    let (base_str, base_bytes) = all_dummy_bytes();
    let data = Decode!(base_bytes.as_ref(), types_all::Snapshot).unwrap();
    let re_bytes = Encode!(&data).unwrap();
    let re_str = format!("{:x?}", re_bytes);
    (
        base_bytes == re_bytes,
        base_str,
        re_str
    )
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn all_default_re_bytes() -> (bool, String, String) {
    let (base_str, base_bytes) = all_default_bytes();
    let data = Decode!(base_bytes.as_ref(), types_all::Snapshot).unwrap();
    let re_bytes = Encode!(&data).unwrap();
    let re_str = format!("{:x?}", re_bytes);
    (
        base_bytes == re_bytes,
        base_str,
        re_str
    )
}

// stable_memory with cell
#[ic_cdk::query]
#[candid::candid_method(query)]
fn stable_get_datum() -> types_all::Snapshot {
    let datum = STABLE_DATUM.with(|p| p.borrow().get().clone());
    datum
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn stable_add_datum_from_dummy() {
    let (_, base_bytes) = all_dummy_bytes();
    let data = Decode!(base_bytes.as_ref(), types_all::Snapshot).unwrap();
    stable_add_datum(data);
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn stable_add_datum_from_default() {
    let (_, base_bytes) = all_default_bytes();
    let data = Decode!(base_bytes.as_ref(), types_all::Snapshot).unwrap();
    stable_add_datum(data);
}
fn stable_add_datum(data: types_all::Snapshot) {
    STABLE_DATUM.with(|mem| {
        mem.borrow_mut().set(data.clone()).unwrap();
    });
}

// stable_memory with vec
#[ic_cdk::query]
#[candid::candid_method(query)]
fn stable_vec_get_last_data() -> types_all::Snapshot {
    let datum = STABLE_DATA.with(|mem| {
        let borrowed_mem = mem.borrow();
        let len = borrowed_mem.len();
        let res = borrowed_mem.get(len - 1);
        res.unwrap().clone()
    });
    datum
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn stable_vec_get_data(n: u64) -> types_all::Snapshot {
    let datum = STABLE_DATA.with(|mem| {
        let borrowed_mem = mem.borrow();
        let res = borrowed_mem.get(n as u64);
        res.unwrap().clone()
    });
    datum
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn stable_vec_add_datum_from_dummy() {
    let (_, base_bytes) = all_dummy_bytes();
    let data = Decode!(base_bytes.as_ref(), types_all::Snapshot).unwrap();
    stable_vec_add_data(data);
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn stable_vec_add_datum_from_default() {
    let (_, base_bytes) = all_default_bytes();
    let data = Decode!(base_bytes.as_ref(), types_all::Snapshot).unwrap();
    stable_vec_add_data(data);
}
fn stable_vec_add_data(datum: types_all::Snapshot) {
    STABLE_DATA.with(|mem| {
        mem.borrow_mut().push(&datum).unwrap();
    });
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn stable_vec_get_last_bytes() -> types_all::SnapshotBytes {
    let datum = STABLE_BYTES.with(|mem| {
        let borrowed_mem = mem.borrow();
        let len = borrowed_mem.len();
        let res = borrowed_mem.get(len - 1);
        res.unwrap().clone()
    });
    datum
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn stable_vec_add_bytes_from_dummy() {
    let (_, base_bytes) = all_dummy_bytes();
    let data = types_all::SnapshotBytes(base_bytes);
    stable_vec_add_bytes(data);
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn stable_vec_add_bytes_from_default() {
    let (_, base_bytes) = all_default_bytes();
    let data = types_all::SnapshotBytes(base_bytes);
    stable_vec_add_bytes(data);
}
fn stable_vec_add_bytes(datum: types_all::SnapshotBytes) {
    STABLE_BYTES.with(|mem| {
        mem.borrow_mut().push(&datum).unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }
}