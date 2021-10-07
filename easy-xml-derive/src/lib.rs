extern crate proc_macro;

#[proc_macro_derive(XmlDeserialize, attributes(easy_xml))]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    match de::expand_derive(&ast) {
        Ok(expanded) => expanded.into(),
        Err(msg) => panic!("{}", msg),
    }
}

mod de;
mod se;
mod utils;
