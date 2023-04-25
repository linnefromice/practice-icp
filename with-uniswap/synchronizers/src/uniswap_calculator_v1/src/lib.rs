mod debug;

use candid::Principal;
use ic_cdk::update;
use ic_cdk_macros::query;
use std::cell::RefCell;

thread_local! {
    static READER_CANISTERS: RefCell<Vec<Principal>> = RefCell::default();
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
fn register_reader(reader: Principal) {
    READER_CANISTERS.with(|value| {
        let mut readers = value.borrow_mut();
        readers.push(reader);
    });
}
