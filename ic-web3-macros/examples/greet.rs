use ic_web3_macros::cross_canister_call_func;

type CallCanisterArgs = (String, String);
type CallCanisterResponse = Result<String, String>;

cross_canister_call_func!("greet", CallCanisterArgs, CallCanisterResponse);

fn main() {}