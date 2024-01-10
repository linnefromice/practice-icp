use std::cell::RefCell;

mod types;

thread_local! {
    static HEAP_DATUM: RefCell<Option<types::Snapshot>> = RefCell::new(None);
    static HEAP_DATA: RefCell<Vec<types::Snapshot>> = RefCell::new(Vec::new());
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_data_by_json() -> types::SnapshotValue {
    let raw = get_raw_data_by_json();
    serde_json::from_str(&raw).unwrap()
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_snapshot_by_json() -> types::Snapshot {
    let value = get_data_by_json();
    types::Snapshot {
        value,
        timestamp: 0,
    }
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_raw_data_by_json() -> String {
    include_str!("../../v3pool.json").to_string()
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_datum() -> Option<types::Snapshot> {
    let datum = HEAP_DATUM.with(|mem| {
        mem.borrow().clone()
    });
    datum
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_last_data() -> types::Snapshot {
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
fn get_selected_data(n: u64) -> types::Snapshot {
    let datum = HEAP_DATA.with(|mem| {
        let borrowed_mem = mem.borrow();
        let res = borrowed_mem.get(n as usize);
        res.unwrap().clone()
    });
    datum
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn add_datum() {
    let datum = get_snapshot_by_json();
    HEAP_DATUM.with(|mem| {
        *mem.borrow_mut() = Some(datum.clone());
    });
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn add_data() {
    let datum = get_snapshot_by_json();
    HEAP_DATA.with(|mem| {
        mem.borrow_mut().push(datum);
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