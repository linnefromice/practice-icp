mod store;

use candid::Principal;
use ic_web3_macros::{cross_canister_call_func, manage_single_state, setup_func};

type CallCanisterArgs = (String, String);
type CallCanisterResponse = Result<String, String>;
cross_canister_call_func!("greet", CallCanisterArgs, CallCanisterResponse);

#[ic_cdk::update]
async fn call(canister_id: String, name: String, msg: String) -> Result<String, String> {
    ic_cdk::println!("calling");
    let canister_id = Principal::from_text(canister_id).unwrap();
    call_greet(canister_id, (name, msg)).await
}

#[ic_cdk::query]
fn greet(name: String, msg: String) -> Result<String, String> {
    ic_cdk::println!("called");
    Ok(format!("Hello, {}! {}", name, msg))
}

manage_single_state!("rpc", String);
manage_single_state!("chain_id", u8);
manage_single_state!("dst_address", String);

setup_func!({
    rpc: String,
    chain_id: u8,
    dst_address: String,
});

#[cfg(test)]
mod test_lib {
    use super::*;

    #[test]
    fn test_setup() {
        let rpc = String::from("rpc");
        let chain_id = 1;
        let dst_address = String::from("dst_address");
        setup(rpc.clone(), chain_id, dst_address.clone());
        assert_eq!(get_rpc(), rpc);
        assert_eq!(get_chain_id(), chain_id);
        assert_eq!(get_dst_address(), dst_address);
    }
}
