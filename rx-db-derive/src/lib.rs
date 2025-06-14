use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// TODO: 检查id是否存在，判定id是否为Option

#[proc_macro_derive(Record)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    quote!(
        impl IRecord for #ident {
            fn get_id(&self) -> Option<RecordId> {
                self.id
            }

            fn set_id(&mut self, id: RecordId) {
                self.id = Some(id)
            }
        }
    )
    .into()
}
