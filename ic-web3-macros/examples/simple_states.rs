use ic_web3_macros::manage_single_state;

manage_single_state!("last_timestamp", u64, 100);
manage_single_state!("last_result", String);

fn main() {}