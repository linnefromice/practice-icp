use ic_web3_macros::setup_func;

pub fn set_rpc(_value: String) {}
pub fn set_chain_id(_value: u8) {}
pub fn set_dst_address(_value: String) {}

setup_func!({
    rpc: String,
    chain_id: u8,
    dst_address: String,
});

fn main() {}