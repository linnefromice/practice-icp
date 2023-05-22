mod test_functions {
    use ic_web3_macros::{manage_single_state, setup_func};

    manage_single_state!("rpc", String);
    manage_single_state!("chain_id", u8);
    manage_single_state!("dst_address", String);
    setup_func!({
        rpc: String,
        chain_id: u8,
        dst_address: String,
    });

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