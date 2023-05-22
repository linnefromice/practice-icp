use proc_macro::TokenStream;
use syn::{parse::Parser};
use quote::{quote, format_ident};

pub fn cross_canister_call_func(input: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    let args = parser.parse(input).expect("Failed to parse input");
    if args.len() != 3 {
        panic!("Expected exactly 3 arguments");
    }

    let fn_name = match &args[0] {
        syn::Expr::Lit(lit) => {
            if let syn::Lit::Str(lit_str) = &lit.lit {
                lit_str.value()
            } else {
                panic!("Expected a string literal for the function name");
            }
        }
        _ => panic!("Expected a string literal for the function name"),
    };
    let call_fn_name = format_ident!("call_{}", fn_name);
    let args_type = &args[1];
    let result_type = &args[2];
    
    let output = quote! {
        async fn #call_fn_name(
            canister_id: Principal,
            call_args: #args_type,
        ) -> #result_type {
            let res = ic_cdk::api::call::call::<_, (#result_type,)>(canister_id, #fn_name, call_args)
                .await
                .map_err(|e| format!("call error: {:?}", e))?;
            res.0
        }
    };
    output.into()
}