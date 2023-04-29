use std::{
    ops::{Div, Mul},
    str::FromStr,
};

use ic_web3::{
    contract::{tokens::Tokenize, Contract, Options},
    ic::{get_public_key, pubkey_to_address, KeyInfo},
    transports::ICHttp,
    types::{Address, SignedTransaction, U256, U64},
    Web3,
};

use crate::{chain_id, eth_gas_price, eth_tx_count, key_name, rpc_url};

pub fn default_derivation_key() -> Vec<u8> {
    ic_cdk::id().as_slice().to_vec()
}

pub fn generate_web3_client(
    max_resp: Option<u64>,
    cycles: Option<u64>,
) -> Result<Web3<ICHttp>, String> {
    match ICHttp::new(rpc_url().as_str(), max_resp, cycles) {
        Ok(v) => Ok(Web3::new(v)),
        Err(e) => Err(e.to_string()),
    }
}

fn generate_contract_client(
    w3: Web3<ICHttp>,
    contract_addr: &str,
    abi: &[u8],
) -> Result<Contract<ICHttp>, String> {
    let contract_address = Address::from_str(contract_addr).unwrap();
    Contract::from_json(w3.eth(), contract_address, abi)
        .map_err(|e| format!("init contract failed: {}", e))
}

pub async fn public_key() -> Result<Vec<u8>, String> {
    get_public_key(None, vec![default_derivation_key()], key_name().to_string()).await
}

pub fn to_ethereum_address(pub_key: Vec<u8>) -> Result<Address, String> {
    pubkey_to_address(&pub_key)
}

pub async fn ethereum_address() -> Result<Address, String> {
    let pub_key = public_key().await?;
    to_ethereum_address(pub_key)
}

pub async fn sign(
    w3: Web3<ICHttp>,
    contract_addr: &str,
    abi: &[u8],
    func: &str,
    params: impl Tokenize,
    gas_coefficient: Option<(u128, u128)>,
    gas_limit: Option<u128>,
) -> Result<SignedTransaction, String> {
    let contract = generate_contract_client(w3.clone(), contract_addr, abi)?;
    let canister_addr = ethereum_address().await?;

    let tx_count = eth_tx_count(w3.clone(), canister_addr).await?;
    let gas_price = eth_gas_price(w3.clone()).await?;
    let options = Options::with(|op| {
        op.nonce = Some(tx_count);
        op.transaction_type = Some(U64::from(2)); // EIP1559_TX_ID
        if gas_coefficient.is_some() {
            let gas_coefficient_value = gas_coefficient.unwrap();
            op.gas_price = Some(
                gas_price
                    .mul(U256::from(gas_coefficient_value.0))
                    .div(U256::from(gas_coefficient_value.1)),
            );
        } else {
            op.gas_price = Some(gas_price);
        }

        // temp
        if let Some(gas_limit_value) = gas_limit {
            op.gas = Some(U256::from(gas_limit_value))
        }
    });

    match contract
        .sign(
            func,
            params,
            options,
            hex::encode(canister_addr),
            KeyInfo {
                derivation_path: vec![default_derivation_key()],
                key_name: key_name().to_string(),
            },
            chain_id(),
        )
        .await
    {
        Ok(v) => Ok(v),
        Err(msg) => Err(format!("sign failed: {}", msg)),
    }
}
