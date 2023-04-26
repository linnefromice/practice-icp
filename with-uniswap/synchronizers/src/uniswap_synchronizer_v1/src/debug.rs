use ic_cdk::{query, update};

use crate::{call_calculate_average_exchange_rate, indexer_canister_id, oracle_address, rpc_url};

#[query]
fn debug_greet(text: String) -> String {
    format!("Hello, world!, Hello, {}", text)
}
#[query]
fn debug_get_rpc_url() -> String {
    rpc_url()
}
#[query]
fn debug_get_oracle_address() -> String {
    oracle_address()
}
#[query]
fn debug_get_indexer_canister_id() -> String {
    indexer_canister_id().to_string()
}
#[update]
async fn debug_call_calculate_average_exchange_rate(
    from: Option<u32>,
    to: Option<u32>,
    precision: u8,
) -> String {
    let canister_id = indexer_canister_id();
    call_calculate_average_exchange_rate(canister_id, from, to, precision).await
}
