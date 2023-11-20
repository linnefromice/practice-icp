use ic_cdk_macros::init;

thread_local! {
    static NAME: std::cell::RefCell<String> = std::cell::RefCell::new("".to_string());
    static AGE: std::cell::RefCell<u32> = std::cell::RefCell::new(0);
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet_with_msg(name: String, msg: String) -> String {
    format!("{} {}", greet(name), msg)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_player() -> (String, u32) {
    (
        NAME.with(|n| n.borrow().clone()),
        AGE.with(|a| a.borrow().clone())
    )
}
#[ic_cdk::update]
#[candid::candid_method]
fn set_player(name: String, age: u32) {
    NAME.with(|n| *n.borrow_mut() = name);
    AGE.with(|a| *a.borrow_mut() = age);
}

#[init]
fn init() {
    NAME.with(|n| *n.borrow_mut() = "Anonymous".to_string());
    AGE.with(|a| *a.borrow_mut() = 99);
}

#[cfg(test)]
mod tests {
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }
}