use std::{cell::RefCell, collections::BTreeMap};

use ic_stable_structures::{memory_manager::{MemoryManager, VirtualMemory, MemoryId}, DefaultMemoryImpl};

mod types;

type MemoryType = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // heap
    static HEAP_DATUM: RefCell<Option<types::Snapshot>> = RefCell::new(None);
    static HEAP_DATA: RefCell<Vec<types::Snapshot>> = RefCell::new(Vec::new());

    // stable memory
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static STABLE_DATUM: RefCell<ic_stable_structures::Cell<types::Snapshot, MemoryType>> = RefCell::new(ic_stable_structures::Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), types::Snapshot::default()).unwrap());
    static STABLE_DATA: RefCell<ic_stable_structures::Vec<types::Snapshot, MemoryType>> = RefCell::new(ic_stable_structures::Vec::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))).unwrap());
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn raw_get_data() -> types::SnapshotValue {
    let raw = raw_data_by_json();
    serde_json::from_str(&raw).unwrap()
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_ticks_size_by_json() -> usize {
    let datum = get_snapshot_by_json();
    datum.value.result.ticks.len()
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_snapshot_by_json() -> types::Snapshot {
    let value = raw_get_data();
    types::Snapshot {
        value,
        timestamp: ic_cdk::api::time() / (1000 * 1000000),
    }
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_tick_filtered_snapshot_by_json(count: u64) -> types::Snapshot {
    let mut value = raw_get_data();
    let ticks_size = value.result.ticks.len();
    value.result.ticks = value.result.ticks.into_iter().skip(ticks_size - count as usize).collect();
    // temp
    value.result.liquidity = Default::default();
    value.result.tick_current = Default::default();
    value.jsonrpc = Default::default();
    types::Snapshot {
        value,
        timestamp: ic_cdk::api::time() / (1000 * 1000000),
    }
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn raw_data_by_json() -> String {
    include_str!("../../v3pool.json").to_string()
}

// heap
#[ic_cdk::query]
#[candid::candid_method(query)]
fn heap_get_datum() -> Option<types::Snapshot> {
    let datum = HEAP_DATUM.with(|mem| {
        mem.borrow().clone()
    });
    datum
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn heap_get_last_data() -> types::Snapshot {
    let datum = HEAP_DATA.with(|mem| {
        let borrowed_mem = mem.borrow();
        let len = borrowed_mem.len();
        let res = borrowed_mem.get(len - 1);
        res.unwrap().clone()
    });
    datum
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn heap_get_selected_data(n: u64) -> types::Snapshot {
    let datum = HEAP_DATA.with(|mem| {
        let borrowed_mem = mem.borrow();
        let res = borrowed_mem.get(n as usize);
        res.unwrap().clone()
    });
    datum
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn heap_add_datum() {
    let datum = get_snapshot_by_json();
    HEAP_DATUM.with(|mem| {
        *mem.borrow_mut() = Some(datum.clone());
    });
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn heap_add_data() {
    let datum = get_snapshot_by_json();
    HEAP_DATA.with(|mem| {
        mem.borrow_mut().push(datum);
    });
}

// stable_memory
#[ic_cdk::query]
#[candid::candid_method(query)]
fn stable_get_datum() -> types::Snapshot {
    let datum = STABLE_DATUM.with(|p| p.borrow().get().clone());
    datum
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn stable_get_last_data() -> types::Snapshot {
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
fn stable_get_selected_data(n: u64) -> types::Snapshot {
    let datum = STABLE_DATA.with(|mem| {
        let borrowed_mem = mem.borrow();
        let res = borrowed_mem.get(n as u64);
        res.unwrap().clone()
    });
    datum
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn stable_add_datum() {
    let datum = get_snapshot_by_json();
    STABLE_DATUM.with(|mem| {
        mem.borrow_mut().set(datum.clone()).unwrap();
    });
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn stable_add_data(is_default: bool) {
    let datum = is_default
        .then(|| types::Snapshot::default())
        .unwrap_or_else(get_snapshot_by_json);
    STABLE_DATA.with(|mem| {
        mem.borrow_mut().push(&datum).unwrap();
    });
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn stable_add_data_dummy() {
    let datum = dummy_snapshot();
    STABLE_DATA.with(|mem| {
        mem.borrow_mut().push(&datum).unwrap();
    });
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn stable_add_data_with_ticks_count(count: u64) {
    let datum = get_tick_filtered_snapshot_by_json(count);
    STABLE_DATA.with(|mem| {
        mem.borrow_mut().push(&datum).unwrap();
    });
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn dummy_snapshot() -> types::Snapshot {
    let mut datum = types::Snapshot::default();
    datum.timestamp = ic_cdk::api::time() / (1000 * 1000000);
    datum.value.id = "1".to_string();
    // datum.value.jsonrpc = "2.0".to_string(); // <- Causes of fail?
    datum.value.result = types::ResultV3Pool {
        ticks: BTreeMap::new(),
        tick_current: 0, // -77_456
        liquidity: "".to_string(), // "0x1d2f091ff09fb67174738".to_string(),
        token0: "0x6b175474e89094c44da98b954eedeac495271d0f".to_string(),
        address: "0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8".to_string(),
        sqrt_ratio_x96: "0x55376cd2ad05b815780ecfb".to_string(),
        tick_spacing: 60
    };
    datum
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