use std::cell::RefCell;

use candid::{Decode, Encode};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, Storable, Vec as StableVec,
};

type MemoryType = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static SNAPSHOT_VEC: std::cell::RefCell<StableVec<Snapshot, MemoryType>> = std::cell::RefCell::new(StableVec::init(
        MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(1)))
    ).unwrap());
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize)]
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

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize)]
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
fn vec_select_all() -> Vec<Snapshot> {
    SNAPSHOT_VEC.with(|vec| vec.borrow().iter().collect())
}
#[ic_cdk::update]
fn vec_insert(transfer: Transfer) {
    let datum = Snapshot {
        value: transfer.clone(),
        timestamp: ic_cdk::api::time(),
    };
    SNAPSHOT_VEC
        .with(|vec| vec.borrow_mut().push(&datum))
        .unwrap();
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
