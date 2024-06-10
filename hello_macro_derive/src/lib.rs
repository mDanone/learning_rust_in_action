use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};


#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let get = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name))
            }
        }
    };
    get.into()
}