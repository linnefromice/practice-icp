use std::{cell::RefCell, ops::Deref};
use candid::CandidType;
use ic_cdk::{api, update, query};

#[derive(Clone, Default, CandidType)]
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
fn debug_update_state(answer: i128, started_at: u64, updated_at: u64) -> Round {
    update_state_internal(answer, started_at, updated_at)
}

#[query]
fn debug_latest_round_id() -> u128 {
    LATEST_ROUND_ID.with(|value| (*value.borrow()).clone())
}
#[query]
fn debug_round(idx: u128) -> Round {
    ROUNDS.with(|rounds| {
        let rounds = rounds.borrow();
        rounds[idx as usize].clone()
    })
}
#[query]
fn debug_rounds_length() -> u128 {
    ROUNDS.with(|rounds| rounds.borrow().len()) as u128
}
#[query]
fn debug_synced_latest_round_id() -> u128 {
    SYNCED_LATEST_ROUND_ID.with(|value| (*value.borrow()).clone())
}
