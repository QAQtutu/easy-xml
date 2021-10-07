use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Field;

use crate::utils::*;

pub fn expand_derive_struct(
    ast: &syn::DeriveInput,
    data: &syn::DataStruct,
) -> Result<TokenStream, String> {
    let name = &ast.ident;

    for f in &data.fields {
        TypeWapper::new(&f.ty).type_check();
    }

    //变量声明
    let var_declare_token: TokenStream = data
        .fields
        .iter()
        .map(|field| var_declare_token(field))
        .collect();

    // 从属性值捕获
    let attribute_fields: TokenStream = data
        .fields
        .iter()
        .filter(|x| {
            let attrs = Attributes::new(&x.attrs);
            return attrs.attribute && attrs.val == false;
        })
        .map(|x| {
            return get_value_from_attribute_token(x);
        })
        .collect();

    // 从节点捕获
    let node_fields: TokenStream = data
        .fields
        .iter()
        .filter(|x| {
            let attrs = Attributes::new(&x.attrs);
            return attrs.attribute == false && attrs.val == false;
        })
        .map(|x| {
            return get_value_from_node_token(x);
        })
        .collect();

    //变量名重绑定
    let var_re_bind: TokenStream = data
        .fields
        .iter()
        .map(|x| {
            return var_re_bind(x);
        })
        .collect();

    let vars: TokenStream = data
        .fields
        .iter()
        .map(|x| {
            let ident = x.ident.as_ref().unwrap();
            return quote! {
              #ident,
            };
        })
        .collect();

    Ok(quote! {
      impl easy_xml::XmlDeserialize for #name{
        fn deserialize(element: &easy_xml::XmlElement) -> Result<Self, easy_xml::de::Error>
        where
            Self: Sized {

            #var_declare_token

            match element {
              easy_xml::XmlElement::Node(node) => {
                  for attr in &node.attributes {
                    let name = &attr.name;
                    #attribute_fields
                  }
                  for element in &node.elements {
                    match element {
                      easy_xml::XmlElement::Text(text) => {
                          // Value
                      }
                      easy_xml::XmlElement::Node(node) => {
                          let name = &node.name;
                          #node_fields
                      }
                      _ => {}
                    }
                  }
              }
              easy_xml::XmlElement::Text(text) => {}
              _ => {}
            }

            #var_re_bind

            Ok(
              #name{
                #vars
              }
            )
        }
      }

    })
}

// 变量声明
// Variable declare Token
/// like  let a:Option<String> = None;
pub fn var_declare_token(field: &Field) -> TokenStream {
    let val_name = field.ident.as_ref().unwrap();
    let ty = TypeWapper::new(&field.ty);
    let token = {
        if ty.has_vec() {
            ((&ty.ty).into_token_stream(), quote! {Vec::new()})
        } else if ty.has_option() {
            let type_token = (&ty.ty).into_token_stream();
            (quote! {Box<#type_token>}, quote! {Box::new(None)})
        } else {
            let type_token = (&ty.ty).into_token_stream();
            (quote! {Box<Option<#type_token>>}, quote! {Box::new(None)})
        }
    };

    let var_type = token.0;
    let var_value = token.1;

    quote! {
      let mut #val_name:#var_type =  #var_value ;
    }
}

// 从节点中捕获变量
pub fn get_value_from_node_token(field: &Field) -> TokenStream {
    let val_name = field.ident.as_ref().unwrap();

    let attrs = Attributes::new(&field.attrs);

    let owned_name_match = owned_name_match(val_name, &attrs);

    let var_instance = get_var_instance(field);

    quote! {
      if #owned_name_match {
        #var_instance
      }
    }
}

// 从属性值中捕获变量
pub fn get_value_from_attribute_token(field: &Field) -> TokenStream {
    let val_name = field.ident.as_ref().unwrap();
    let attrs = Attributes::new(&field.attrs);

    let owned_name_match = owned_name_match(val_name, &attrs);

    let var_instance = get_var_instance(field);
    quote! {
      if #owned_name_match {
        let element = easy_xml::XmlElement::Text(attr.value.clone());

        #var_instance
      }
    }
}

pub fn get_var_instance(field: &Field) -> TokenStream {
    let ty = TypeWapper::new(&field.ty);

    let is_vec = ty.has_vec();

    let ty = {
        if is_vec {
            ty.next_type().unwrap()
        } else {
            ty
        }
    };
    let ty_token = (&ty.ty).into_token_stream();
    let full_path = ty.full_path();
    let token = TokenStream::from_str(full_path.as_str()).unwrap();

    let val_name = field.ident.as_ref().unwrap();

    if is_vec {
        return quote! {
          let field___val : #ty_token =  #token::deserialize(&element)?;
          #val_name.push(field___val);
        };
    } else {
        let instance = match ty.has_option() {
            true => quote! {field___val},
            false => quote! { Some(field___val)},
        };

        return quote! {
          let field___val : #ty_token =  #token::deserialize(&element)?;
          *#val_name =  #instance;
        };
    }
}

pub fn var_re_bind(field: &Field) -> TokenStream {
    let tw = TypeWapper::new(&field.ty);
    let val_name = field.ident.as_ref().unwrap();

    if tw.has_vec() {
        return quote! {};
    } else if tw.has_option() {
        return quote! {
          let #val_name = *#val_name;
        };
    }

    let msg = format!("Field {} has no value!", val_name);

    quote! {
      let #val_name = match *#val_name{
        Some(val) =>val,
        None =>  return Err(easy_xml::de::Error::Other(#msg.to_string())),
      };
    }
}
