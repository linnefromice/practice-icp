use std::cell::RefCell;
use candid::CandidType;
use ic_cdk::{api, update, query};
use ic_cdk_timers::TimerId;

#[derive(Clone, Debug, Default, CandidType)]
pub struct Round {
    pub round_id: u128,
    pub answer: i128,
    pub started_at: u64,
    pub updated_at: u64
}

thread_local! {
    static LATEST_ROUND_ID: RefCell<u128> = RefCell::default();
    static ROUNDS: RefCell<Vec<Round>> = RefCell::default();
    static SYNCED_LATEST_ROUND_ID: RefCell<u128> = RefCell::default();
    static TIMER_ID: RefCell<TimerId> = RefCell::default(); // for debug
}

#[update]
fn update_state(answer: i128) -> Round {
    let timestamp = api::time();
    update_state_internal(answer, timestamp, timestamp)
}
fn update_state_internal(answer: i128, started_at: u64, updated_at: u64) -> Round {
    let incremented_round_id = LATEST_ROUND_ID.with(|val| {
        let mut mut_ref = val.borrow_mut();
        *mut_ref += 1;
        mut_ref.clone()
    });
    let round = Round {
        round_id: incremented_round_id,
        answer,
        started_at,
        updated_at
    };
    ROUNDS.with(|rounds| rounds.borrow_mut().push(round.clone()));
    round
}

#[update]
fn periodic_sync_state() {
    periodic_sync_state_internal(30, 5)
}

fn periodic_sync_state_internal(interval_secs: u64, max_run_unit: u128) {
    let interval = std::time::Duration::from_secs(interval_secs);
    let max_run_unit_owned = std::sync::Arc::new(max_run_unit);

    let timer_id = ic_cdk_timers::set_timer_interval(interval, move || {
        ic_cdk::println!("[START] Synchronization");
        let synced_latest_round_id = get_synced_latest_round_id();
        let latest_round_id = get_latest_round_id();
        if synced_latest_round_id == latest_round_id {
            ic_cdk::println!("Already synced: round_id is {:?}", synced_latest_round_id);
            return
        }
        let not_synced = latest_round_id - synced_latest_round_id;
        let run_unit = if not_synced > *max_run_unit_owned { *max_run_unit_owned } else { not_synced };

        for i in 0..run_unit {
            let round = get_round(synced_latest_round_id+1+i-1);
            ic_cdk::println!("sync round: {:?}", round);
        }

        SYNCED_LATEST_ROUND_ID.with(|value| *value.borrow_mut() += run_unit);
        ic_cdk::println!("[FINISH] Synchronization");
    });

    TIMER_ID.with(|value| *value.borrow_mut() = timer_id);
}

fn get_latest_round_id() -> u128 {
    LATEST_ROUND_ID.with(|value| (*value.borrow()).clone())
}
fn get_round(idx: u128) -> Round {
    ROUNDS.with(|rounds| {
        let rounds = rounds.borrow();
        rounds[idx as usize].clone()
    })
}
fn get_synced_latest_round_id() -> u128 {
    SYNCED_LATEST_ROUND_ID.with(|value| (*value.borrow()).clone())
}

#[update]
fn debug_update_state(answer: i128, started_at: u64, updated_at: u64) -> Round {
    update_state_internal(answer, started_at, updated_at)
}

#[query]
fn debug_latest_round_id() -> u128 {
    get_latest_round_id()
}
#[query]
fn debug_round(idx: u128) -> Round {
    get_round(idx)
}
#[query]
fn debug_rounds_length() -> u128 {
    ROUNDS.with(|rounds| rounds.borrow().len()) as u128
}
#[query]
fn debug_synced_latest_round_id() -> u128 {
    get_synced_latest_round_id()
}
#[update]
fn debug_periodic_sync_state(interval_secs: u64, max_run_unit: u128) {
    periodic_sync_state_internal(interval_secs, max_run_unit)
}
#[update]
fn debug_stop_timer() {
    let timer_id = TIMER_ID.with(|value| value.borrow().clone());
    ic_cdk_timers::clear_timer(timer_id);
}