pub const UNISWAPV3_POOL_ABI: &[u8] = include_bytes!("../../abi/UniswapV3Pool.json");

pub const DEFAULT_FETCH_INTERVAL_BY_SEC: u64 = 10;

// alchemy = 250
// infura = 150
// quicknode = 500
pub const BASE_MAX_RESP_BYTES_FOR_HEADER: u64 = 500;
pub const MAX_RESP_BYTES_ONE_SLOT: u64 = 50;
pub const MAX_RESP_BYTES_TO_CALL_SLOT0: u64 = 450;
pub const MAX_RESP_BYTES_TO_CALL_OBSERVATION: u64 = 300;
