use proc_macro::TokenStream;
use syn::{parse::Parser, Type, LitStr, parse_macro_input};
use quote::{quote};

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

    let var_ident = proc_macro2::Ident::new(&var_name.to_uppercase(), proc_macro2::Span::call_site());
    let get_ident = proc_macro2::Ident::new(&format!("get_{}", var_name), proc_macro2::Span::call_site());
    let set_ident = proc_macro2::Ident::new(&format!("set_{}", var_name), proc_macro2::Span::call_site());

    let output = generate_single_state(var_type, var_init, var_ident, get_ident, set_ident);
    output.into()
}
fn generate_single_state(
    var_type: Type,
    var_init: proc_macro2::TokenStream,
    var_ident: proc_macro2::Ident,
    get_ident: proc_macro2::Ident,
    set_ident: proc_macro2::Ident,
) -> proc_macro2::TokenStream {
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
    output
}

struct ManageVecArgs {
    name: LitStr,
    ty: Type,
}
impl syn::parse::Parse for ManageVecArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let ty = input.parse()?;
        Ok(ManageVecArgs { name, ty })
    }
}
pub fn manage_vec_state(input: TokenStream) -> TokenStream {
    let ManageVecArgs { name, ty } = parse_macro_input!(input as ManageVecArgs);
    
    let state_name = name.value();
    let state_upper_name = syn::Ident::new(&format!("{}S", state_name.to_uppercase()), name.span());
    let get_vec_func = syn::Ident::new(&format!("get_{}s", state_name), name.span());
    let get_len_func = syn::Ident::new(&format!("{}s_len", state_name), name.span());
    let get_elem_func = syn::Ident::new(&format!("get_{}", state_name), name.span());
    let set_elem_func = syn::Ident::new(&format!("set_{}", state_name), name.span());

    let expanded = quote! {
        thread_local! {
            static #state_upper_name: std::cell::RefCell<Vec<#ty>> = std::cell::RefCell::new(Vec::new());
        }

        pub fn #get_vec_func() -> Vec<#ty> {
            #state_upper_name.with(|state| state.borrow().clone())
        }

        pub fn #get_len_func() -> usize {
            #state_upper_name.with(|state| state.borrow().len())
        }

        pub fn #get_elem_func(idx: usize) -> #ty {
            #state_upper_name.with(|state| state.borrow()[idx].clone())
        }

        pub fn #set_elem_func(value: #ty) {
            #state_upper_name.with(|state| state.borrow_mut().push(value));
        }
    };

    TokenStream::from(expanded)
}

struct ManageMapArgs {
    name: LitStr,
    key_ty: Type,
    val_ty: Type,
}
impl syn::parse::Parse for ManageMapArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let key_ty = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let val_ty = input.parse()?;
        Ok(ManageMapArgs { name, key_ty, val_ty })
    }
}
pub fn manage_map_state(input: TokenStream) -> TokenStream {
    let ManageMapArgs { name, key_ty, val_ty } = parse_macro_input!(input as ManageMapArgs);

    let state_name = name.value();
    let state_upper_name = syn::Ident::new(&format!("{}S", state_name.to_uppercase()), name.span());
    let get_len_func = syn::Ident::new(&format!("{}s_len", state_name), name.span());
    let get_elem_func = syn::Ident::new(&format!("get_{}", state_name), name.span());
    let set_elem_func = syn::Ident::new(&format!("set_{}", state_name), name.span());

    let expanded = quote! {
        thread_local! {
            static #state_upper_name: std::cell::RefCell<std::collections::HashMap<#key_ty, #val_ty>> = std::cell::RefCell::new(std::collections::HashMap::new());
        }

        pub fn #get_len_func() -> usize {
            #state_upper_name.with(|state| state.borrow().len())
        }

        pub fn #get_elem_func(key: #key_ty) -> #val_ty {
            #state_upper_name.with(|state| state.borrow().get(&key).cloned().expect("key not found"))
        }

        pub fn #set_elem_func(key: #key_ty, value: #val_ty) {
            #state_upper_name.with(|state| state.borrow_mut().insert(key, value));
        }
    };

    TokenStream::from(expanded)
}