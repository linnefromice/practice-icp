use candid::{Principal, candid_method};
use ic_cdk_macros::update;
use ic_cdk_timers::TimerId;

use crate::{call_get_period_range_realized_volatility, types::CallCanisterArgs, store::{clean_snapshot_data, get_timer_id, set_timer_id}};

#[update]
#[candid_method(update)]
async fn debug_call_get_period_range_realized_volatility(
    target_canister_id: String,
    data_resource_canister_id: String,
    token0_decimals: u8,
    token1_decimals: u8,
    precision: u8,
    time_unit: u32,
    back_terms: Option<u8>,
) -> Result<String, String> {
    let target_canister_id = Principal::from_text(target_canister_id).unwrap();
    call_get_period_range_realized_volatility(
        target_canister_id,
        CallCanisterArgs {
            data_resource_canister_id,
            token0_decimals,
            token1_decimals,
            precision,
            time_unit,
            back_terms,
        },
    )
    .await
}

#[update]
#[candid_method(update)]
pub fn debug_clean_snapshot_data() {
    clean_snapshot_data();
}

#[update]
#[candid_method(update)]
pub fn debug_stop_task() {
    let timer_id = get_timer_id();
    ic_cdk_timers::clear_timer(timer_id);
    set_timer_id(TimerId::default());
}
