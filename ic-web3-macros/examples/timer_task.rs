use ic_web3_macros::timer_task_func;

static hello: fn() -> () = || {};
timer_task_func!("set_task", "hello");

fn main() {}
