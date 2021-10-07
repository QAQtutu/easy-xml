use proc_macro2::TokenStream;

pub mod expand_enum;
pub mod expand_struct;

pub fn expand_derive(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
    let data = &ast.data;

    match &*data {
        syn::Data::Struct(data) => {
            return expand_struct::expand_derive_struct(ast, data);
        }
        syn::Data::Enum(data) => {
            return expand_enum::expand_derive_enum(ast, data);
        }
        syn::Data::Union(_) => {
            panic!("{}", "不支持 Union")
        }
    };
}
