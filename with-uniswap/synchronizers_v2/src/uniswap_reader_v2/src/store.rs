use ic_cdk_timers::TimerId;
use std::cell::RefCell;

use crate::types::Price;

thread_local! {
    static PRICES: RefCell<Vec<Price>> = RefCell::default();
    static TIMER_ID: RefCell<TimerId> = RefCell::default();
    static RPC_URL: RefCell<String> = RefCell::default();
    static POOL_ADDRESS: RefCell<String> = RefCell::default();
}

pub fn get_timer_id() -> TimerId {
    TIMER_ID.with(|value| *value.borrow())
}
pub fn set_timer_id(timer_id: TimerId) {
    TIMER_ID.with(|value| *value.borrow_mut() = timer_id);
}

pub fn get_rpc_url() -> String {
    RPC_URL.with(|value| value.borrow().clone())
}
pub fn set_rpc_url(url: String) {
    RPC_URL.with(|value| *value.borrow_mut() = url);
}

pub fn get_pool_address() -> String {
    POOL_ADDRESS.with(|value| value.borrow().clone())
}
pub fn set_pool_address(pool_addr: String) {
    POOL_ADDRESS.with(|value| *value.borrow_mut() = pool_addr);
}

pub fn prices() -> Vec<Price> {
    PRICES.with(|val| val.borrow().clone())
}
pub fn last_price() -> Option<Price> {
    PRICES.with(|val| val.borrow().last().cloned())
}
pub fn prices_length() -> u64 {
    PRICES.with(|val| val.borrow().len()) as u64
}
pub fn price(idx: u64) -> Option<Price> {
    PRICES.with(|val| val.borrow().get(idx as usize).cloned())
}
pub fn add_price(price: Price) {
    PRICES.with(|val| val.borrow_mut().push(price));
}
