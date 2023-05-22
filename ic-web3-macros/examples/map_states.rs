use ic_web3_macros::manage_map_state;

manage_map_state!("balance", String, u64);
manage_map_state!("username", u64, String);

fn main() {}
