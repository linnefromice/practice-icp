use std::cell::RefCell;

use candid::{Decode, Encode};
use chainsight_cdk_macros::{stable_memory_for_vec, StableMemoryStorable};
use ic_cdk::api::stable::stable64_size;
use ic_stable_structures::{
    memory_manager::{MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};

#[derive(
    Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, StableMemoryStorable,
)]
#[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)] // temp: max_size
pub struct Snapshot {
    pub value: SnapshotValue,
    pub timestamp: u64,
}
type SnapshotValue = u64;

type MemoryType = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

stable_memory_for_vec!("snapshot", Snapshot, 1, true);
// Function with dependencies
fn proxy() -> candid::Principal {
    candid::Principal::anonymous()
}

#[ic_cdk::update]
fn insert_snapshot(value: SnapshotValue) {
    let snapshot = Snapshot {
        value,
        timestamp: ic_cdk::api::time() / 1000000,
    };
    add_snapshot(snapshot);
}

#[ic_cdk::update]
fn insert_snapshot_bulk(value: SnapshotValue, count: u64) {
    let timestamp = ic_cdk::api::time() / 1000000;
    let snapshots = (0..count)
        .map(|_| Snapshot { value, timestamp })
        .collect::<Vec<_>>();
    for (i, snapshot) in snapshots.into_iter().enumerate() {
        if i % 10000 == 0 {
            ic_cdk::println!("Inserting snapshot: {}", i);
        }
        add_snapshot(snapshot);
    }
}

#[ic_cdk::query]
fn current_used_memory() -> u64 {
    stable64_size()
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }

    #[derive(
        Debug,
        Clone,
        candid::CandidType,
        candid::Deserialize,
        serde::Serialize,
        StableMemoryStorable,
    )]
    #[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)] // temp: max_size
    pub struct SnapshotTransfer {
        pub value: Transfer,
        pub timestamp: u64,
    }
    #[derive(
        Debug,
        Clone,
        candid::CandidType,
        candid::Deserialize,
        serde::Serialize,
        StableMemoryStorable,
    )]
    #[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)] // temp: max_size
    pub struct Transfer {
        pub from: String,
        pub to: String,
        pub amount: u64,
    }

    #[derive(
        Debug,
        Clone,
        candid::CandidType,
        candid::Deserialize,
        serde::Serialize,
        StableMemoryStorable,
    )]
    #[stable_mem_storable_opts(max_size = 10000, is_fixed_size = false)] // temp: max_size
    pub struct SnapshotLatestRound {
        pub value: (u64, String, String, String, u64),
        pub timestamp: u64,
    }

    #[test]
    fn test_size() {
        let encoded = Encode!(&Snapshot {
            value: 1,
            timestamp: 1,
        })
        .unwrap();
        println!("Encoded size: {}", encoded.len());
        let encoded = Encode!(&Snapshot {
            value: 1_000_000_000,
            timestamp: 1_000_000_000,
        })
        .unwrap();
        println!("Encoded size: {}", encoded.len());

        let encoded = Encode!(&(1_000_000_000u64, 1_000_000_000u64)).unwrap();
        println!("Encoded size: {}", encoded.len());

        let encoded = Encode!(&SnapshotTransfer {
            value: Transfer {
                from: "0x0000000000000000000000000000000000000000".to_string(),
                to: "0x0000000000000000000000000000000000000000".to_string(),
                amount: 1_000_000_000,
            },
            timestamp: 1_000_000_000,
        })
        .unwrap();
        println!("Encoded size > SnapshotTransfer: {}", encoded.len());
        let encoded = Encode!(&SnapshotLatestRound {
            value: (
                1_000_000_000,
                "1000000000".to_owned(),
                "1000000000".to_owned(),
                "1000000000".to_owned(),
                1_000_000_000
            ),
            timestamp: 1_000_000_000,
        })
        .unwrap();
        println!("Encoded size > SnapshotLatestRound: {}", encoded.len());

        let encoded = Encode!(&SnapshotLatestRound {
            value: (
                1_000_000_000,
                "1".to_owned(),
                "1".to_owned(),
                "1".to_owned(),
                1_000_000_000
            ),
            timestamp: 1_000_000_000,
        })
        .unwrap();
        println!("Encoded size > SnapshotLatestRound: {}", encoded.len());
    }
}
