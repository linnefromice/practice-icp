use candid::CandidType;
use ic_web3::{
    contract::{tokens::Detokenize, Error},
    ethabi::Token,
    types::U256,
};
pub struct Slot0(U256, i32, u16, u16, u16, u8, bool);

impl Detokenize for Slot0 {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self(
            tokens.get(0).unwrap().clone().into_uint().unwrap(),
            tokens.get(1).unwrap().clone().into_int().unwrap().as_u128() as i32,
            tokens.get(2).unwrap().clone().into_uint().unwrap().as_u32() as u16,
            tokens.get(3).unwrap().clone().into_uint().unwrap().as_u32() as u16,
            tokens.get(4).unwrap().clone().into_uint().unwrap().as_u32() as u16,
            tokens.get(5).unwrap().clone().into_uint().unwrap().as_u32() as u8,
            tokens.get(6).unwrap().clone().into_bool().unwrap(),
        ))
    }
}

impl Slot0 {
    pub fn to_candid(&self) -> CandidSlot0 {
        CandidSlot0 {
            sqrt_price_x96: self.0.to_string(),
            tick: self.1,
            observation_index: self.2,
            observation_cardinality: self.3,
            observation_cardinality_next: self.4,
            fee_protocol: self.5,
            unlocked: self.6,
        }
    }
}

#[derive(CandidType)]
pub struct CandidSlot0 {
    sqrt_price_x96: String,
    tick: i32,
    observation_index: u16,
    observation_cardinality: u16,
    observation_cardinality_next: u16,
    fee_protocol: u8,
    unlocked: bool,
}

pub struct Observation(u32, i64, U256, bool);

impl Detokenize for Observation {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self(
            tokens.get(0).unwrap().clone().into_uint().unwrap().as_u32(),
            tokens.get(1).unwrap().clone().into_int().unwrap().as_u128() as i64,
            tokens.get(2).unwrap().clone().into_uint().unwrap(),
            tokens.get(3).unwrap().clone().into_bool().unwrap(),
        ))
    }
}

impl Observation {
    pub fn to_candid(&self) -> CandidObservation {
        CandidObservation {
            block_timestamp: self.0,
            tick_cumulative: self.1,
            liquidity_cumulative: self.2.to_string(),
            initialized: self.3,
        }
    }
}

#[derive(CandidType)]
pub struct CandidObservation {
    pub block_timestamp: u32,
    pub tick_cumulative: i64,
    pub liquidity_cumulative: String,
    pub initialized: bool,
}
