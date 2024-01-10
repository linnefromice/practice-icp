use std::{collections::BTreeMap, borrow::Cow};

use candid::{Decode, Encode};
use ic_stable_structures::{Storable, BoundedStorable};

pub fn dummy() -> Snapshot {
    Snapshot {
        timestamp: ic_cdk::api::time() / (1000 * 1000000),
        value: SnapshotValue {
            id: "1".to_string(),
            jsonrpc: "2.0".to_string(),
            result: ResultV3Pool {
                ticks: BTreeMap::new(),
                tick_current: -77_456,
                liquidity: "0x1d2f091ff09fb67174738".to_string(),
                token0: "0x6b175474e89094c44da98b954eedeac495271d0f".to_string(),
                address: "0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8".to_string(),
                sqrt_ratio_x96: "0x55376cd2ad05b815780ecfb".to_string(),
                tick_spacing: 60
            }
        }
    }
}

#[derive(Debug, Clone, Default, candid::CandidType, candid::Deserialize, serde::Serialize)]
pub struct SnapshotBytes(pub Vec<u8>);
impl Storable for SnapshotBytes {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        ic_cdk::println!("from_bytes");
        ic_cdk::println!("{:x?}", bytes);
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let res = Encode!(self).unwrap();
        ic_cdk::println!("to_bytes");
        ic_cdk::println!("{:x?}", res);
        Cow::Owned(res)
    }
}
impl BoundedStorable for SnapshotBytes {
    const MAX_SIZE: u32 = 100;
    const IS_FIXED_SIZE: bool = false;
}


#[derive(Debug, Clone, Default, candid::CandidType, candid::Deserialize, serde::Serialize)]
pub struct Snapshot {
    pub value: SnapshotValue,
    pub timestamp: u64,
}
impl Storable for Snapshot {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        ic_cdk::println!("from_bytes");
        ic_cdk::println!("{:x?}", bytes);
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let res = Encode!(self).unwrap();
        ic_cdk::println!("to_bytes");
        ic_cdk::println!("{:x?}", res);
        Cow::Owned(res)
    }
}
impl BoundedStorable for Snapshot {
    const MAX_SIZE: u32 = 100;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Debug, Clone, Default, candid::CandidType, candid::Deserialize, serde::Serialize)]
pub struct SnapshotValue {
    pub jsonrpc: String,
    pub id: String,
    pub result: ResultV3Pool,
}

#[derive(Debug, Clone, Default, candid::CandidType, candid::Deserialize, serde::Serialize)]
pub struct ResultV3Pool {
    pub address: String,
    pub token0: String,
    pub sqrt_ratio_x96: String,
    pub liquidity: String,
    pub tick_current: i32,
    pub tick_spacing: i32,
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