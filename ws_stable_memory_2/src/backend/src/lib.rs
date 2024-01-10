mod types;

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