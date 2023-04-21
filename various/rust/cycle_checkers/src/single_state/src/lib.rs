use std::cell::RefCell;

thread_local! {
    static FIRST_STATE: RefCell<u8> = RefCell::default();
}

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
