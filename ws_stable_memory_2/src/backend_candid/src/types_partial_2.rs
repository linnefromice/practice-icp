use std::{collections::BTreeMap, borrow::Cow};

use candid::{Decode, Encode};
use ic_stable_structures::{Storable, BoundedStorable};

#[derive(Debug, Clone, Default, candid::CandidType, candid::Deserialize, serde::Serialize)]
pub struct Snapshot {
    pub value: BTreeMap<String, String>,
}
impl Storable for Snapshot {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}
impl BoundedStorable for Snapshot {
    const MAX_SIZE: u32 = 100;
    const IS_FIXED_SIZE: bool = false;
}
