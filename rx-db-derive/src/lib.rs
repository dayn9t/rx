use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

// TODO: 检查id是否存在，判定id是否为Option

#[proc_macro_derive(Record)]
pub fn derive_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    quote!(
        impl IRecord for #ident {
            type RecordId = usize;
            fn get_id(&self) -> Option<usize> {
                self.id
            }

            fn set_id(&mut self, id: &usize) {
                self.id = Some(*id)
            }
        }
    )
    .into()
}

#[proc_macro_derive(RecordSid)]
pub fn derive_record_sid(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    quote!(
        impl IRecord for #ident {
            type RecordId = String;
            fn get_id(&self) -> Option<String> {
                self.id.clone()
            }

            fn set_id(&mut self, id: &String) {
                self.id = Some(id.clone())
            }
        }
    )
    .into()
}
