use std::cell::RefCell;

thread_local! {
    static HEAP_COUNT: RefCell<u64> = RefCell::new(0);
}

#[ic_cdk::query]
fn hello() -> String {
    "Hello!".to_string()
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get() -> u64 {
    let datum = HEAP_COUNT.with(|mem| {
        mem.borrow().clone()
    });
    ic_cdk::println!("get: {}", datum);
    datum
}
#[ic_cdk::update]
#[candid::candid_method(update)]
fn increment() {
    HEAP_COUNT.with(|mem| {
        let mut count = mem.borrow_mut();
        *count += 1;
    });
}

#[cfg(test)]
mod tests {
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }
}
