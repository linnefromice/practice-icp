use std::cell::RefCell;

thread_local! {
    static STATE1: RefCell<u8> = RefCell::default();
    static STATE2: RefCell<u8> = RefCell::default();
    static STATE3: RefCell<u8> = RefCell::default();
    static STATE4: RefCell<u8> = RefCell::default();
    static STATE5: RefCell<u8> = RefCell::default();
    static STATE6: RefCell<u8> = RefCell::default();
    static STATE7: RefCell<u8> = RefCell::default();
    static STATE8: RefCell<u8> = RefCell::default();
    static STATE9: RefCell<u8> = RefCell::default();
    static STATE10: RefCell<u8> = RefCell::default();
    static STATE11: RefCell<u8> = RefCell::default();
    static STATE12: RefCell<u8> = RefCell::default();
    static STATE13: RefCell<u8> = RefCell::default();
    static STATE14: RefCell<u8> = RefCell::default();
    static STATE15: RefCell<u8> = RefCell::default();
    static STATE16: RefCell<u8> = RefCell::default();
    static STATE17: RefCell<u8> = RefCell::default();
    static STATE18: RefCell<u8> = RefCell::default();
    static STATE19: RefCell<u8> = RefCell::default();
    static STATE20: RefCell<u8> = RefCell::default();
}

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
