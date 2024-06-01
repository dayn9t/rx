use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CompleteBy)]
pub fn complete_by_derive(input: TokenStream) -> TokenStream {
    // 解析输入
    let input = parse_macro_input!(input as DeriveInput);

    // 获取结构体名称
    let name = &input.ident;

    // 获取所有字段的名称
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        panic!("CompleteBy can only be derived for structs with named fields");
    };

    // 为每个字段生成一个complete_by调用
    let field_names = fields.iter().map(|f| &f.ident);
    let expansions = field_names.map(|field_name| {
        quote! {
            self.#field_name = self.#field_name.clone().or_else(|| other.#field_name.clone());
        }
    });

    // 生成输出的TokenStream
    let expanded = quote! {
        impl CompleteBy for #name {
            fn complete_by(&mut self, other: &Self) {
                #(#expansions)*
            }
        }
    };

    // 返回生成的TokenStream
    TokenStream::from(expanded)
}
