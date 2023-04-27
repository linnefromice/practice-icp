use ic_web3::{contract::Contract, transports::ICHttp, types::Address, Web3};
use std::str::FromStr;

use crate::rpc_url;

pub fn generate_uniswapv3pool_client(
    w3: Web3<ICHttp>,
    contract_addr: &str,
    abi: &[u8],
) -> Result<Contract<ICHttp>, String> {
    let contract_address = Address::from_str(contract_addr).unwrap();
    Contract::from_json(w3.eth(), contract_address, abi)
        .map_err(|e| format!("init contract failed: {}", e))
}

pub fn generate_web3_client(
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<Web3<ICHttp>, String> {
    match ICHttp::new(&rpc_url(), max_resp, cycles) {
        Ok(v) => Ok(Web3::new(v)),
        Err(e) => Err(e.to_string()),
    }
}
