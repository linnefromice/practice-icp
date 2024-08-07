use std::cell::RefCell;

use candid::{Decode, Encode};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap as StableBTreeMap, DefaultMemoryImpl, Storable,
};

type MemoryType = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static SNAPSHOT_MAP: std::cell::RefCell<StableBTreeMap<u64, Snapshot, MemoryType>> = std::cell::RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(2)))
    ));
}

#[derive(Debug, Clone, PartialEq, candid::CandidType, candid::Deserialize, serde::Serialize)]
pub struct Transfer {
    pub from: String,
    pub to: String,
    pub amount: u64,
}
impl Storable for Transfer {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}

#[derive(Debug, Clone, PartialEq, candid::CandidType, candid::Deserialize, serde::Serialize)]
pub struct Snapshot {
    pub value: SnapshotValue,
    pub timestamp: u64,
}
impl Storable for Snapshot {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}
type SnapshotValue = Transfer;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn xxx_len() -> u64 {
    SNAPSHOT_MAP.with(|map| map.borrow().len())
}
#[ic_cdk::query]
fn get_xxxs() -> Vec<Snapshot> {
    let first = if let Some(res) = first_key_value() {
        res
    } else {
        return vec![];
    };
    if let Some((to, _)) = last_key_value() {
        range(first.0, to + 1)
    } else {
        vec![first.1]
    }
}
#[ic_cdk::query]
fn get_last_xxx() -> Snapshot {
    let last = last_key_value().unwrap();
    last.1
}
#[ic_cdk::query]
fn get_top_xxxs(n: u64) -> Vec<Snapshot> {
    let data_len = xxx_len();
    let from = if data_len > n { data_len + 1 - n } else { 0 };
    range(from, data_len + 1)
}
#[ic_cdk::query]
fn get_xxx(idx: u64) -> Snapshot {
    SNAPSHOT_MAP.with(|map| map.borrow().get(&idx).unwrap().clone())
}
#[ic_cdk::update]
fn add_xxx(value: Snapshot) {
    let idx = xxx_len() + 1;
    SNAPSHOT_MAP.with(|map| map.borrow_mut().insert(idx, value));
}

fn range(from: u64, to: u64) -> Vec<Snapshot> {
    SNAPSHOT_MAP.with(|map| {
        map.borrow()
            .range(from..to)
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    })
}

fn last_key_value() -> Option<(u64, Snapshot)> {
    let res = SNAPSHOT_MAP.with(|map| map.borrow().last_key_value().clone());
    res.clone()
}
fn first_key_value() -> Option<(u64, Snapshot)> {
    SNAPSHOT_MAP.with(|map| map.borrow().first_key_value().clone())
}

// sample
// #[ic_cdk::query]
// fn btree_select_all() -> Vec<Snapshot> {
//     let to = btree_count() + 1;
//     btree_range(1, to)
// }
// #[ic_cdk::query]
// fn btree_range(from: u64, to: u64) -> Vec<Snapshot> {
//     SNAPSHOT_MAP.with(|map| {
//         map.borrow()
//             .range(from..to)
//             .into_iter()
//             .map(|(_, v)| v)
//             .collect()
//     })
// }
// #[ic_cdk::query]
// fn btree_count() -> u64 {
//     SNAPSHOT_MAP.with(|map| map.borrow().len())
// }
// #[ic_cdk::update]
// fn btree_insert(transfer: Transfer) -> Option<Snapshot> {
//     let datum = Snapshot {
//         value: transfer.clone(),
//         timestamp: ic_cdk::api::time(),
//     };
//     let idx = btree_count() + 1;
//     SNAPSHOT_MAP.with(|map| map.borrow_mut().insert(idx, datum))
// }

#[cfg(test)]
mod tests {
    use super::*;
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }

    #[test]
    fn test_storage() {
        assert_eq!(xxx_len(), 0);

        let transfer = Transfer {
            from: "Alice".to_string(),
            to: "Bob".to_string(),
            amount: 100,
        };
        let datum_1 = Snapshot {
            value: transfer.clone(),
            timestamp: 1,
        };
        let datum_2 = Snapshot {
            value: transfer.clone(),
            timestamp: 2,
        };
        let datum_3 = Snapshot {
            value: transfer.clone(),
            timestamp: 3,
        };
        add_xxx(datum_1.clone());
        add_xxx(datum_2.clone());
        add_xxx(datum_3.clone());

        assert_eq!(xxx_len(), 3);
        assert_eq!(&get_xxx(1), &datum_1);
        assert_eq!(&get_xxx(2), &datum_2);
        assert_eq!(&get_xxx(3), &datum_3);
        assert_eq!(&get_last_xxx(), &datum_3);
        assert_eq!(get_top_xxxs(1), vec![datum_3.clone()]);
        assert_eq!(get_top_xxxs(2), vec![datum_2.clone(), datum_3.clone()]);
        assert_eq!(
            get_top_xxxs(3),
            vec![datum_1.clone(), datum_2.clone(), datum_3.clone()]
        );
        assert_eq!(
            get_xxxs(),
            vec![datum_1.clone(), datum_2.clone(), datum_3.clone()]
        );

        let datum_4 = Snapshot {
            value: transfer.clone(),
            timestamp: 4,
        };
        let datum_5 = Snapshot {
            value: transfer.clone(),
            timestamp: 5,
        };
        add_xxx(datum_4.clone());
        add_xxx(datum_5.clone());
        assert_eq!(xxx_len(), 5);
        assert_eq!(&get_last_xxx(), &datum_5);
        assert_eq!(get_top_xxxs(1), vec![datum_5.clone()]);
        assert_eq!(get_top_xxxs(2), vec![datum_4.clone(), datum_5.clone()]);
        assert_eq!(
            get_xxxs(),
            vec![
                datum_1.clone(),
                datum_2.clone(),
                datum_3.clone(),
                datum_4.clone(),
                datum_5.clone()
            ]
        );
    }
}
