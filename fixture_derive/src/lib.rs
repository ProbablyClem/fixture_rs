use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Fixture)]
pub fn derive_fixture(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = input.data
    {
        fields.named.iter().map(|f| {
            let field_name = &f.ident;
            quote! {
                #field_name: Fixture::fixture()
            }
        })
    } else {
        unimplemented!("Fixture can only be derived for structs with named fields")
    };

    let expanded = quote! {
        impl Fixture for #name {
            fn fixture() -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
