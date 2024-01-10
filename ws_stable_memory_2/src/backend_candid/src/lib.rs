use candid::{Encode, Decode};

mod types_all;
mod types_partial_1;
mod types_partial_2;

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

#[cfg(test)]
mod tests {
    use super::*;
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }
}