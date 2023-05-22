mod manage_single_state {
    use ic_web3_macros::manage_single_state;

    manage_single_state!("last_timestamp", u64, 100);
    manage_single_state!("latest_result", String);

    #[test]
    fn test_u64() {
        assert_eq!(get_last_timestamp(), 100);
        set_last_timestamp(200);
        assert_eq!(get_last_timestamp(), 200);
    }

    #[test]
    fn test_string() {
        assert_eq!(get_latest_result(), String::from(""));
        set_latest_result(String::from("UPDATED"));
        assert_eq!(get_latest_result(), String::from("UPDATED"));
    }
}

mod manage_vec_state {
    use ic_web3_macros::manage_vec_state;

    manage_vec_state!("result", String);

    #[test]
    fn test_vec() {
        assert_eq!(results_len(), 0);
        let datum1 = String::from("RESULT1"); 
        let datum2 = String::from("RESULT2");
        set_result(datum1.clone());
        set_result(datum2.clone());
        assert_eq!(results_len(), 2);
        assert_eq!(get_results(), vec![datum1.clone(), datum2.clone()]);
        assert_eq!(get_result(0), datum1.clone());
        assert_eq!(get_result(1), datum2.clone());
    }
}

mod manage_map_state {
    use ic_web3_macros::manage_map_state;

    manage_map_state!("balance", String, u64);
    manage_map_state!("username", u64, String);

    #[test]
    fn test_balances() {
        assert_eq!(balances_len(), 0);
        let datum1 = String::from("BALANCE1"); 
        let datum2 = String::from("BALANCE2");
        set_balance(datum1.clone(), 100);
        set_balance(datum2.clone(), 200);
        assert_eq!(balances_len(), 2);
        assert_eq!(get_balance(datum1.clone()), 100);
        assert_eq!(get_balance(datum2.clone()), 200);
    }

    #[test]
    fn test_usernames() {
        assert_eq!(usernames_len(), 0);
        let datum1 = String::from("USERNAME1"); 
        let datum2 = String::from("USERNAME2");
        set_username(1, datum1.clone());
        set_username(2, datum2.clone());
        assert_eq!(usernames_len(), 2);
        assert_eq!(get_username(1), datum1.clone());
        assert_eq!(get_username(2), datum2.clone());
    }
}

