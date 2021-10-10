use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::utils::{self, Field};
use crate::utils::{owned_name_match, Attributes};

pub fn expand_derive_enum(
    ast: &syn::DeriveInput,
    data: &syn::DataEnum,
) -> Result<TokenStream, String> {
    let enum_name = &ast.ident;

    let get_from_text = get_from_text(enum_name, data);

    let get_from_node = get_from_node(enum_name, data);

    Ok(quote! {

      impl easy_xml::XmlDeserialize for #enum_name {
        fn deserialize(element: &easy_xml::XmlElement) -> Result<Self, easy_xml::de::Error>
        where
            Self: Sized,
        {
            match element {
                easy_xml::XmlElement::Text(text) => match text.as_str() {
                    #get_from_text
                    _ => return Err(easy_xml::de::Error::Other("".to_string())),
                },
                easy_xml::XmlElement::Node(node) => {
                  let name = &node.name;

                  #get_from_node

                  return Err(easy_xml::de::Error::Other("".to_string()))
                },
                easy_xml::XmlElement::Whitespace(_) => {return Err(easy_xml::de::Error::Other("".to_string()))},
                easy_xml::XmlElement::Comment(_) => {return Err(easy_xml::de::Error::Other("".to_string()))},
                easy_xml::XmlElement::CData(_) => {return Err(easy_xml::de::Error::Other("".to_string()))},
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
                    let fields = (&named.named)
                        .into_iter()
                        .map(|f| {
                            let f = utils::Field::from_named(f);

                            return f;
                        })
                        .collect::<Vec<_>>();

                    for f in &fields {
                        f.check()
                    }

                    code_for_named_and_unnamed(true, enum_name, ident, fields)
                }

                syn::Fields::Unnamed(unnamed) => {
                    let mut index = 1;

                    let fields = (&unnamed.unnamed)
                        .into_iter()
                        .map(|f| {
                            let f = utils::Field::from_unnamed(f, index);
                            index += 1;
                            return f;
                        })
                        .collect::<Vec<_>>();

                    for f in &fields {
                        f.check()
                    }

                    code_for_named_and_unnamed(false, enum_name, ident, fields)
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

fn code_for_named_and_unnamed(
    is_struct: bool,
    enum_name: &Ident,
    variant_name: &Ident,
    fields: Vec<Field>,
) -> TokenStream {
    // let mut f_0: Box<Option<String>> = Box::new(None);
    // let mut f_1: Vec<String> = Vec::new();
    let code_for_declare = utils::build_code_for_declare(&fields);

    // {
    //     *f_1 = Some(String::deserialize(&element)?);
    // }
    let code_for_flatten = utils::build_code_for_flatten(&fields);
    //   {
    //     let mut text = String::new();
    //     element.text(&mut text);
    //     let element = easy_xml::XmlElement::Text(text);
    //     *f_0 = Some(String::deserialize(&element)?);
    //   }
    let code_for_text = utils::build_code_for_text(&fields);

    // for attr in &node.attributes {
    //     let name = &attr.name;
    //     if true && "T" == &name.local_name {
    //         let element = easy_xml::XmlElement::Text(attr.value.clone());
    //         *f_5 = Some(String::deserialize(&element)?);
    //     }
    // }
    let code_for_attribute = utils::build_code_for_attribute(&fields);

    let code_for_children = utils::build_code_for_children(&fields);

    let var_rebind = utils::var_rebind(&fields);

    let var_collect = utils::var_collect(&fields);

    let var_collect = match is_struct {
        true => quote! {
          return Ok(
            #enum_name :: #variant_name{#var_collect}
          );
        },
        false => quote! {
          return Ok(
            #enum_name :: #variant_name(#var_collect)
          );
        },
    };

    quote! {
      #code_for_declare

      #code_for_flatten

      #code_for_text

      #code_for_attribute

      #code_for_children

      #var_rebind

      #var_collect
    }
}
