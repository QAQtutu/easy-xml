use std::str::FromStr;

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::utils::{self, Attributes, Field};

pub fn expand_derive_enum(
    ast: &syn::DeriveInput,
    data: &syn::DataEnum,
) -> Result<TokenStream, String> {
    let enum_name = &ast.ident;

    let code_for_text = build_code_for_text(enum_name, data);

    let code_for_node = build_code_for_node(enum_name, data);

    Ok(quote! {
      impl easy_xml::XmlSerialize for #enum_name {
        fn serialize(&self, element: &mut easy_xml::XmlElement)
        where
            Self: Sized,
          {
            match element {
              easy_xml::XmlElement::Text(text) => {
                  #code_for_text
              }
              easy_xml::XmlElement::Node(node) => {
                  #code_for_node
              },
              _ => {}
          }
        }
      }
    })
}

fn build_code_for_text(enum_name: &Ident, data: &syn::DataEnum) -> TokenStream {
    let code: TokenStream = (&data.variants)
        .into_iter()
        .map(|v| {
            let var_name = &v.ident;
            let var_name_str = var_name.to_string();
            return match &v.fields {
                syn::Fields::Named(named) => {
                    let vars: TokenStream = (&named.named)
                        .into_iter()
                        .map(|f| {
                            let field = f.ident.as_ref().unwrap().to_string();
                            let ident =
                                TokenStream::from_str(format!("{}:f_{}", field, field).as_str())
                                    .unwrap();
                            quote! {
                              #ident,
                            }
                        })
                        .collect();
                    quote! {
                      #enum_name::#var_name{#vars} => {text.push_str(#var_name_str)}
                    }
                }
                syn::Fields::Unnamed(unnamed) => {
                    let vars: TokenStream = (&unnamed.unnamed)
                        .into_iter()
                        .map(|_f| quote! {_,})
                        .collect();
                    quote! {
                      #enum_name::#var_name(#vars) => {text.push_str(#var_name_str)}
                    }
                }
                syn::Fields::Unit => quote! {
                  #enum_name::#var_name => {text.push_str(#var_name_str)}
                },
            };
        })
        .collect();
    quote! {
      match self {
        #code
      };
    }
}

fn build_code_for_node(enum_name: &Ident, data: &syn::DataEnum) -> TokenStream {
    let code: TokenStream = (&data.variants)
        .into_iter()
        .map(|v| {
            let var_name = &v.ident;
            let attrs = Attributes::new(&v.attrs);

            match &v.fields {
                syn::Fields::Named(named) => {
                    let fields = (&named.named)
                        .into_iter()
                        .map(|f| {
                            let f = utils::Field::from_named(f);
                            return f;
                        })
                        .collect::<Vec<_>>();

                    let vars = utils::se_build_code_for_fields(&fields);
                    let code_variant = utils::se_build_code_for_set_tag(&v.ident, &attrs);
                    let code_named = code_for_named_and_unnamed(&fields);
                    return quote! {
                      #enum_name::#var_name{#vars} => {
                        #code_variant
                        #code_named
                      },
                    };
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
                    let vars: TokenStream = utils::se_build_code_for_fields(&fields);

                    let code_variant = utils::se_build_code_for_set_tag(&v.ident, &attrs);

                    let code_unnamed = code_for_named_and_unnamed(&fields);

                    return quote! {
                      #enum_name::#var_name(#vars) => {
                        #code_variant

                        #code_unnamed
                      },
                    };
                }
                syn::Fields::Unit => {
                    let code_variant = utils::se_build_code_for_set_tag(&v.ident, &attrs);
                    return quote! {
                      #enum_name::#var_name => {
                        #code_variant
                      },
                    };
                }
            }
        })
        .collect();
    quote! {
      match self {
        #code
      }
    }
}

fn code_for_named_and_unnamed(fields: &Vec<Field>) -> TokenStream {
    let code_for_text = utils::se_build_code_for_text(&fields);

    let code_for_flatten = utils::se_build_code_for_flatten(&fields);

    let code_for_attribute = utils::se_build_code_for_attribute(&fields);

    let code_for_node = utils::se_build_code_for_node(&fields);
    quote! {
      #code_for_text

      #code_for_attribute

      #code_for_node

      #code_for_flatten
    }
}
