use candid::Principal;
use ic_cdk::{api::call, init};
use std::cell::RefCell;

mod debug;

thread_local! {
    static RPC_URL: RefCell<String>  = RefCell::default();
    static ORACLE_ADDRESS: RefCell<String> = RefCell::default();
    static INDEXER_CANISTER_ID: RefCell<Principal> = RefCell::new(Principal::anonymous());
}

#[init]
fn init(url: String, oracle_addr: String, indexer: Principal) {
    RPC_URL.with(|value| *value.borrow_mut() = url);
    ORACLE_ADDRESS.with(|value| *value.borrow_mut() = oracle_addr);
    INDEXER_CANISTER_ID.with(|value| *value.borrow_mut() = indexer);
}

fn rpc_url() -> String {
    RPC_URL.with(|value| (value.borrow()).clone())
}
fn oracle_address() -> String {
    ORACLE_ADDRESS.with(|value| (value.borrow()).clone())
}
fn indexer_canister_id() -> Principal {
    INDEXER_CANISTER_ID.with(|value| *value.borrow())
}

async fn call_calculate_average_exchange_rate(
    canister_id: Principal,
    from: Option<u32>,
    to: Option<u32>,
    precision: u8,
) -> String {
    call::call::<_, (String,)>(
        canister_id,
        "calculate_average_exchange_rate",
        (from, to, precision),
    )
    .await
    .unwrap()
    .0
}
