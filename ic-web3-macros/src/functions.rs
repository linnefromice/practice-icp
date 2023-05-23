use proc_macro::TokenStream;
use quote::quote;
use syn::{braced, Result, Ident, Type, Token, punctuated::Punctuated, parse::{Parse, ParseStream, Parser}};

struct SetupArgs {
    fields: Punctuated<NamedField, Token![,]>,
}
impl Parse for SetupArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);
        let fields = Punctuated::parse_terminated(&content)?;
        Ok(SetupArgs { fields })
    }
}
struct NamedField {
    name: Ident,
    _colon_token: Token![:],
    ty: Type,
}
impl Parse for NamedField {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(NamedField {
            name: input.parse()?,
            _colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}
pub fn setup_func(input: TokenStream) -> TokenStream {
    let SetupArgs { fields } = syn::parse_macro_input!(input as SetupArgs);

    let setters: Vec<_> = fields
        .iter()
        .map(|field| Ident::new(&format!("set_{}", field.name), field.name.span()))
        .collect();

    let names: Vec<_> = fields.iter().map(|field| &field.name).collect();
    let types: Vec<_> = fields.iter().map(|field| &field.ty).collect();

    let expanded = quote! {
        #[ic_cdk::update]
        fn setup(#( #names: #types ),*) {
            #( #setters(#names); )*
        }
    };

    TokenStream::from(expanded)
}

pub fn timer_task_func(input: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    let args = parser.parse(input).expect("Failed to parse input");
    if args.len() != 2 {
        panic!("Expected 2 arguments");
    }

    let func_name = match &args[0] {
        syn::Expr::Lit(lit) => {
            if let syn::Lit::Str(lit_str) = &lit.lit {
                syn::Ident::new(&format!("{}", lit_str.value()), lit_str.span())
            } else {
                panic!("Expected a string literal for the variable name");
            }
        }
        _ => panic!("Expected a string literal for the variable name"),
    };
    let called_func_name = match &args[1] {
        syn::Expr::Lit(lit) => {
            if let syn::Lit::Str(lit_str) = &lit.lit {
                syn::Ident::new(&format!("{}", lit_str.value()), lit_str.span())
            } else {
                panic!("Expected a string literal for the variable name");
            }
        }
        _ => panic!("Expected a string literal for the variable name"),
    };

    let timer_state_name = format!("timer_task_{}", called_func_name);
    let set_timer_state_name = syn::Ident::new(&format!("set_timer_task_{}", called_func_name), called_func_name.span());

    let output = quote! {
        ic_web3_macros::manage_single_state!(#timer_state_name, ic_cdk_timers::TimerId);

        #[ic_cdk::update]
        pub fn #func_name(task_interval_secs: u32, delay_secs: u32) {
            let current_time_sec = (ic_cdk::api::time() / (1000 * 1000000)) as u32;
            let round_timestamp = |ts: u32, unit: u32| ts / unit * unit;
            let delay = round_timestamp(current_time_sec, task_interval_secs) + task_interval_secs + delay_secs - current_time_sec;
            ic_cdk_timers::set_timer(std::time::Duration::from_secs(delay as u64), move || {
                let timer_id = ic_cdk_timers::set_timer_interval(std::time::Duration::from_secs(task_interval_secs as u64), || {
                    #called_func_name();
                });
                #set_timer_state_name(timer_id);
            });
        }
    };

    output.into()
}
