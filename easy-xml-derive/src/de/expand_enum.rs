use std::str::FromStr;

use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use crate::de::expand_struct::{
    get_value_from_attribute_token, get_value_from_node_token, get_value_from_text_token,
    var_declare_token, var_re_bind,
};
use crate::utils::{owned_name_match, Attributes};

pub fn expand_derive_enum(
    ast: &syn::DeriveInput,
    data: &syn::DataEnum,
) -> Result<TokenStream, String> {
    let enum_name = &ast.ident;

    for v in &data.variants {
        match &v.fields {
            syn::Fields::Named(named) => {
                println!("Named");
                for _f in &named.named {
                    // println!("{:?}", f.ident);
                }
            }
            syn::Fields::Unnamed(unnamed) => {
                let len = unnamed.unnamed.len();
                if len == 0 {
                    panic!("Unnamed 必须有一个元素! Unnamed must have an element!")
                }
                if len > 1 {
                    panic!("Unnamed 只允许一个元素! Unnamed allows only one element!")
                }
            }
            syn::Fields::Unit => {}
        }
    }

    let get_from_text = get_from_text(enum_name, data);

    let get_from_node = get_from_node(enum_name, data);

    Ok(quote! {

      impl easy_xml::XmlDeserialize for #enum_name {
        fn deserialize(element: &easy_xml::XmlElement) -> Result<Self, de::Error>
        where
            Self: Sized,
        {
            match element {
                easy_xml::XmlElement::Text(text) => match text.as_str() {
                    #get_from_text
                    _ => return Err(de::Error::Other("".to_string())),
                },
                easy_xml::XmlElement::Node(node) => {
                  let name = &node.name;

                  #get_from_node

                  Err(de::Error::Other("".to_string()))
                },
                easy_xml::XmlElement::Whitespace(_) => {return Err(de::Error::Other("".to_string()))},
                easy_xml::XmlElement::Comment(_) => {return Err(de::Error::Other("".to_string()))},
                easy_xml::XmlElement::CData(_) => {return Err(de::Error::Other("".to_string()))},
            }

        }
      }
    })
}

fn get_from_text(enum_name: &Ident, data: &syn::DataEnum) -> TokenStream {
    return (&data.variants)
        .into_iter()
        .filter(|v| match v.fields {
            syn::Fields::Unit => true,
            _ => false,
        })
        .map(|v| {
            let ident = &v.ident;
            let attrs = Attributes::new(&v.attrs);

            let mut tag = String::new();
            if let Some(prefix) = attrs.prefix {
                tag += prefix.as_str();
                tag += ":";
            }
            if let Some(rename) = attrs.rename {
                tag += rename.as_str();
            } else {
                tag += ident.to_string().as_str();
            }

            quote! {
              #tag => Ok(#enum_name::#ident),
            }
        })
        .collect();
}

fn get_from_node(enum_name: &Ident, data: &syn::DataEnum) -> TokenStream {
    let token: TokenStream = (&data.variants)
        .into_iter()
        .map(|v| {
            let ident = &v.ident;
            let attrs = Attributes::new(&v.attrs);
            let owned_name_match = owned_name_match(ident, &attrs);

            let enum_instance = match &v.fields {
                syn::Fields::Named(named) => {
                    //变量声明
                    let var_declare_token: TokenStream = (&named.named)
                        .iter()
                        .map(|field| var_declare_token(field))
                        .collect();

                    // 从节点捕获
                    let node_fields = get_value_from_node_token((&named.named).iter());
                    // 从属性值捕获
                    let attribute_fields = get_value_from_attribute_token((&named.named).iter());
                    // 从文本内容捕获
                    let text_fields = get_value_from_text_token((&named.named).iter());

                    //变量名重绑定
                    let var_re_bind: TokenStream = (&named.named)
                        .iter()
                        .map(|x| {
                            return var_re_bind(x);
                        })
                        .collect();

                    let vars: TokenStream = (&named.named)
                        .iter()
                        .map(|x| {
                            let ident = x.ident.as_ref().unwrap();
                            let var_name =
                                TokenStream::from_str(format!("f_{}", ident.to_string()).as_str())
                                    .unwrap();
                            return quote! {
                              #ident:#var_name,
                            };
                        })
                        .collect();

                    quote! {
                      #var_declare_token

                      #text_fields

                      #attribute_fields

                      #node_fields

                      #var_re_bind

                      return Ok( #enum_name::#ident {#vars});
                    }
                }

                syn::Fields::Unnamed(unnamed) => {
                    let field = unnamed.unnamed.first().unwrap();
                    let ty_token = (&field.ty).into_token_stream();
                    quote! {

                     return Ok( #enum_name::#ident (#ty_token::deserialize(element)?));
                    }
                }
                syn::Fields::Unit => {
                    quote! {
                      return Ok(#enum_name::#ident);
                    }
                }
            };

            quote! {
              if #owned_name_match{
                #enum_instance
              }
            }
        })
        .collect();

    return quote! {
      #token
    };
}
