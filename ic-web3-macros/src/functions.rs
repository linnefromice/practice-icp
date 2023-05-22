use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Result, Ident, Type, Token, punctuated::Punctuated};

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