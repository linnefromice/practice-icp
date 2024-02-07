use ic_cdk_timers::TimerId;

thread_local! {
    static TIMER_ID: std::cell::RefCell<TimerId> = std::cell::RefCell::default();
}

pub fn _get_timer_id() -> TimerId {
    TIMER_ID.with(|state| *state.borrow())
}
pub fn _set_timer_id(value: TimerId) {
    TIMER_ID.with(|state| *state.borrow_mut() = value);
}

#[ic_cdk::query]
#[candid::candid_method(query)]
pub fn get_timer_id() -> String {
    format!("{:?}", _get_timer_id())
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }
}
