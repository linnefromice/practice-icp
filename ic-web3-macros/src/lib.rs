use proc_macro::TokenStream;
use syn::{parse::Parser, Type};
use quote::{quote, format_ident};

#[proc_macro]
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

#[proc_macro]
pub fn manage_single_state(input: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;

    let args = parser.parse(input).expect("Failed to parse input");
    if args.len() != 2 && args.len() != 3 {
        panic!("Expected 2 or 3 arguments");
    }

    let var_name = match &args[0] {
        syn::Expr::Lit(lit) => {
            if let syn::Lit::Str(lit_str) = &lit.lit {
                lit_str.value()
            } else {
                panic!("Expected a string literal for the variable name");
            }
        }
        _ => panic!("Expected a string literal for the variable name"),
    };

    let var_type: Type = match &args[1] {
        syn::Expr::Path(path) => syn::Type::Path(syn::TypePath { qself: None, path: path.path.clone() }),
        _ => panic!("Expected a type for the second argument"),
    };

    let var_init = if args.len() == 3 {
        match &args[2] {
            syn::Expr::Lit(lit) => quote! { #lit },
            _ => panic!("Expected a literal for the initial value"),
        }
    } else {
        quote! { std::default::Default::default() }
    };

    let var_ident = proc_macro2::Ident::new(&var_name, proc_macro2::Span::call_site());
    let get_ident = proc_macro2::Ident::new(&format!("get_{}", var_name), proc_macro2::Span::call_site());
    let set_ident = proc_macro2::Ident::new(&format!("set_{}", var_name), proc_macro2::Span::call_site());

    let output = quote! {
        thread_local! {
            static #var_ident: std::cell::RefCell<#var_type> = std::cell::RefCell::new(#var_init);
        }

        pub fn #get_ident() -> #var_type {
            #var_ident.with(|state| state.borrow().clone())
        }

        pub fn #set_ident(value: #var_type) {
            #var_ident.with(|state| *state.borrow_mut() = value);
        }
    };
    output.into()
}