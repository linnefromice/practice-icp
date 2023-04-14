use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[ic_cdk_macros::init]
fn init(timer_interval_secs: u64) {
    let interval = std::time::Duration::from_secs(timer_interval_secs);
    ic_cdk::println!("Starting a periodic task with interval {interval:?}");
    ic_cdk_timers::set_timer_interval(interval, || {
        let incremented = increment();
        ic_cdk::println!("current is {incremented}");
    });
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade(timer_interval_secs: u64) {
    init(timer_interval_secs);
}

#[ic_cdk_macros::query]
fn counter() -> u64 {
    COUNTER.load(Ordering::Relaxed)
}

fn increment() -> u64 {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}