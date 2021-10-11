use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::{self, Attributes, Field};

pub fn expand_derive_struct(
    ast: &syn::DeriveInput,
    data: &syn::DataStruct,
) -> Result<TokenStream, String> {
    let struct_name = &ast.ident;
    let attrs = Attributes::new(&ast.attrs);

    let fields = (&data.fields)
        .into_iter()
        .map(|f| {
            let f = Field::from_struct(f);
            return f;
        })
        .collect::<Vec<_>>();

    for f in &fields {
        f.check()
    }

    let code_for_root = utils::se_build_code_for_root(&ast.ident, &attrs);

    let code_for_text = utils::se_build_code_for_text(&fields);

    let code_for_flatten = utils::se_build_code_for_flatten(&fields);

    let code_for_attribute = utils::se_build_code_for_attribute(&fields);

    let code_for_node = utils::se_build_code_for_node(&fields);

    Ok(quote! {
      impl easy_xml::XmlSerialize for #struct_name {
        fn serialize(&self, element: &mut easy_xml::XmlElement)
        where
            Self: Sized,
        {

            match element {
                easy_xml::XmlElement::Text(s) => {}
                easy_xml::XmlElement::Node(node) => {

                    #code_for_root

                    #code_for_text

                    #code_for_attribute

                    #code_for_node
                }
                _ => {}
            }
            #code_for_flatten
        }
      }
    })
}
