mod sample_test {
    // cargo test --tests test_sample -- --nocapture
    #[test]
    fn test_sample() {
        let hello = |name: String| -> String { format!("Hello, {}!", name) };
        println!("{}", hello(String::from("pokemon")));
    }
}