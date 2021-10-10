use proc_macro2::TokenStream;
use quote::quote;

use crate::utils;

pub fn expand_derive_struct(
    ast: &syn::DeriveInput,
    data: &syn::DataStruct,
) -> Result<TokenStream, String> {
    let name = &ast.ident;

    let fields = (&data.fields)
        .into_iter()
        .map(|f| {
            let f = utils::Field::from_named(f);
            return f;
        })
        .collect::<Vec<_>>();

    for f in &fields {
        f.check()
    }

    // //变量声明
    let code_for_declare = utils::build_code_for_declare(&fields);

    let code_for_flatten = utils::build_code_for_flatten(&fields);

    // 从文本捕获值
    let code_for_text = utils::build_code_for_text(&fields);

    let code_for_attribute = utils::build_code_for_attribute(&fields);

    let code_for_children = utils::build_code_for_children(&fields);

    let var_rebind = utils::var_rebind(&fields);

    let var_collect = utils::var_collect(&fields);

    Ok(quote! {
      impl easy_xml::XmlDeserialize for #name{
        fn deserialize(element: &easy_xml::XmlElement) -> Result<Self, easy_xml::de::Error>
        where
            Self: Sized {

            //属性变量定义
            #code_for_declare

            #code_for_flatten

            //文本内容捕获
            #code_for_text

            match element {
              easy_xml::XmlElement::Node(node) => {

                  #code_for_attribute

                  #code_for_children
              }
              easy_xml::XmlElement::Text(text) => {}
              _ => {}
            }

            #var_rebind

            Ok(
              #name{
                #var_collect
              }
            )
        }
      }

    })
}
