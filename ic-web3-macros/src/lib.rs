mod functions;
mod states;
mod utils;

use proc_macro::TokenStream;

#[proc_macro]
pub fn cross_canister_call_func(input: TokenStream) -> TokenStream {
    utils::cross_canister_call_func(input)
}

#[proc_macro]
pub fn setup_func(input: TokenStream) -> TokenStream {
    functions::setup_func(input)
}

#[proc_macro]
pub fn timer_task_func(input: TokenStream) -> TokenStream {
    functions::timer_task_func(input)
}

#[proc_macro]
pub fn manage_single_state(input: TokenStream) -> TokenStream {
    states::manage_single_state(input)
}

#[proc_macro]
pub fn manage_vec_state(input: TokenStream) -> TokenStream {
    states::manage_vec_state(input)
}

#[proc_macro]
pub fn manage_map_state(input: TokenStream) -> TokenStream {
    states::manage_map_state(input)
}
