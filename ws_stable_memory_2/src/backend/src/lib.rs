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

#[cfg(test)]
mod tests {
    use super::*;
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }
}