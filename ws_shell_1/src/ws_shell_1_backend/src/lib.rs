use std::cell::RefCell;
use candid::types::number::Nat;

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0));
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn panic(name: String) {
    panic!("Panic, {}!", name);
}

/// Get the value of the counter.
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

/// Increment the value of the counter.
#[ic_cdk::update]
#[candid::candid_method(update)]
fn inc() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn set(val: i64) {
    if val < 0 {
        panic!("Counter must be positive");
    }
    COUNTER.with(|count| *count.borrow_mut() = Nat::from(val as u64));
}

/// Increment the value of the counter.
#[ic_cdk::update]
#[candid::candid_method(update)]
fn reset() {
    COUNTER.with(|counter| *counter.borrow_mut() = Nat::from(0));
}

candid::export_service!();
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }
}