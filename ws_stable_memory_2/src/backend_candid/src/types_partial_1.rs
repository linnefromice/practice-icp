use std::{collections::BTreeMap, borrow::Cow};

use candid::{Decode, Encode};
use ic_stable_structures::{Storable, BoundedStorable};

#[derive(Debug, Clone, Default, candid::CandidType, candid::Deserialize, serde::Serialize)]
pub struct Snapshot {
    pub value: SnapshotValue,
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

#[derive(Debug, Clone, Default, candid::CandidType, candid::Deserialize, serde::Serialize)]
pub struct SnapshotValue {
    pub result: ResultV3Pool,
}

#[derive(Debug, Clone, Default, candid::CandidType, candid::Deserialize, serde::Serialize)]
pub struct ResultV3Pool {
    #[serde(deserialize_with = "from_ticks")]
    pub ticks: BTreeMap<String, Tick>
}

#[derive(Debug, Clone, Default, candid::CandidType, candid::Deserialize, serde::Serialize)]
pub struct Tick {
    pub liquidity_gross: String,
    pub liquidity_net: String,
    pub fee_growth_outside_0x128: String,
    pub fee_growth_outside_1x128: String,
    pub initialized: bool,
}

// Deserializer
pub fn from_ticks<'de, D>(deserializer: D) -> Result<BTreeMap<String, Tick>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_map(CustomVisitor)
}
struct CustomVisitor;
impl<'de> serde::de::Visitor<'de> for CustomVisitor {
    type Value = BTreeMap<String, Tick>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "todo")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>
    {
        let mut result = BTreeMap::new();
        while let Some((k, v)) = map.next_entry::<i64, Tick>()? {
            // ic_cdk::println!("key={}", &k);
            // ic_cdk::println!("value={:?}", &v);
            result.insert(
                k.to_string(),
                v
            );
        }
        Ok(result)
    }
}