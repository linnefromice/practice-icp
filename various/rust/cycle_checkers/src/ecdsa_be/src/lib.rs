mod utils;

use std::str::FromStr;
use candid::CandidType;
use sha2::Digest;
use ic_cdk::{api::management_canister::{http_request::{TransformArgs, HttpResponse}, ecdsa::{SignWithEcdsaArgument, sign_with_ecdsa, SignWithEcdsaResponse, EcdsaCurve, EcdsaKeyId}}};
use ic_cdk_macros::{query, update};
use ic_web3::{ic::{get_eth_addr, KeyInfo}, types::{Address, TransactionParameters, U256, SignedTransaction}, Web3, transports::ICHttp};
use utils::{generate_web3_client, KN_IN_LOCAL, default_derivation_key, get_public_key, CHAIN_ID, pubkey_to_address};

#[query]
fn transform(response: TransformArgs) -> HttpResponse {
    let res = response.response;
    HttpResponse { status: res.status, headers: Vec::default(), body: res.body }
}

#[derive(CandidType)]
pub struct AccountInfo {
    pub address: String
}

#[update]
async fn get_ethereum_address() -> Result<AccountInfo, String> {
    let res = get_ecdsa_public_key().await;
    if let Err(msg) = res { return Err(msg) };
    let pub_key = res.unwrap();

    let res = pubkey_to_address(&pub_key);
    if let Err(msg) = res { return Err(msg) };
    let addr = res.unwrap();

    return Ok(AccountInfo {
        address: format!("0x{}", hex::encode(addr))
    })
}

#[update]
async fn get_ethereum_address_omitted() -> Result<AccountInfo, String> {
    let res = get_eth_addr(None, None, KN_IN_LOCAL.to_string()).await
        .map_err(|e| format!("get_eth_addr failed: {}", e));
    match res {
        Ok(v) => Ok(AccountInfo { address: v.to_string() }),
        Err(e) => Err(e)
    }
}

#[update]
async fn get_transaction_count(
    target_addr: Option<String>
) -> Result<String, String> {
    let w3 = generate_web3_client(
        Some(300),
        None,
    )
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    
    let canister_addr = if target_addr.is_some() {
        Address::from_str(&target_addr.unwrap()).unwrap()
    } else {
        get_eth_addr(None, None, KN_IN_LOCAL.to_string()).await
            .map_err(|e| format!("get_eth_addr failed: {}", e))?
    };

    let result = w3.eth()
        .transaction_count(canister_addr, None)
        .await
        .map_err(|e| format!("get tx count error: {}", e))?;
    Ok(result.to_string())
}

#[update]
async fn get_gas_price() -> Result<String, String> {
    let w3 = generate_web3_client(
        Some(300),
        None,
    )
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;

    let gas_price = w3.eth()
        .gas_price()
        .await
        .map_err(|e| format!("get gas_price error: {}", e))?;
    Ok(gas_price.to_string())
}

#[update]
async fn get_ecdsa_public_key() -> Result<Vec<u8>, String> {
    get_public_key(
        None,
        vec![default_derivation_key()],
        KN_IN_LOCAL.to_string()
    ).await
}

#[update]
async fn sign_message(
    message: String,
) -> Vec<u8> {
    let message_hash = sha2::Sha256::digest(message).to_vec();
    let arg = SignWithEcdsaArgument {
        message_hash: message_hash,
        derivation_path: vec![default_derivation_key()],
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: KN_IN_LOCAL.to_string(),
        }
    };
    let SignWithEcdsaResponse { signature } = sign_with_ecdsa(arg).await.unwrap().0;
    signature
}

#[update]
async fn sign_transfer_native(to: String, value: u64, tx_count: u128, gas_price: u128, max_resp: u64) -> Result<String, String> {
    let w3 = generate_web3_client(
        Some(max_resp),
        None,
    )
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let signed_tx = sign_transfer_native_internal(w3.clone(), to, value, tx_count, gas_price).await.unwrap();
    Ok(format!("0x{}", hex::encode(signed_tx.raw_transaction.0)))
}

#[update]
async fn transfer_native(to: String, value: u64, tx_count: u128, gas_price: u128, max_resp: u64) -> Result<String, String> {
    let w3 = generate_web3_client(
        Some(max_resp),
        None,
    )
        .map_err(|e| format!("generate_web3_client failed: {}", e))?;
    let signed_tx = sign_transfer_native_internal(w3.clone(), to, value, tx_count, gas_price).await.unwrap();
    match w3.eth().send_raw_transaction(signed_tx.raw_transaction).await {
        Ok(txhash) => {
            ic_cdk::println!("txhash: {}", hex::encode(txhash.0));
            Ok(format!("{}", hex::encode(txhash.0)))
        },
        Err(e) => { Err(e.to_string()) },
    }
}

async fn sign_transfer_native_internal(w3: Web3<ICHttp>, to: String, value: u64, tx_count: u128, gas_price: u128) -> Result<SignedTransaction, String> {
    let canister_addr = get_eth_addr(None, None, KN_IN_LOCAL.to_string()).await
        .map_err(|e| format!("get_eth_addr failed: {}", e))?;

    let tx = TransactionParameters {
        to: Some(Address::from_str(&to).unwrap()),
        nonce: Some(U256::from(tx_count)),
        value: U256::from(value),
        gas_price: Some(U256::from(gas_price)),
        gas: U256::from(21000),
        ..Default::default()
    };
    let signed_tx = w3.accounts()
        .sign_transaction(
            tx,
            hex::encode(canister_addr),
            KeyInfo { derivation_path: vec![default_derivation_key()], key_name: KN_IN_LOCAL.to_string() },
            CHAIN_ID
        )
        .await
        .map_err(|e| format!("sign_transaction error: {}", e))?;
    Ok(signed_tx)
}
