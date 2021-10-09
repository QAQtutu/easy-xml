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

    //字段检查
    fields_check((&data.fields).into_iter());

    //变量声明
    let var_declare_token: TokenStream = data
        .fields
        .iter()
        .map(|field| var_declare_token(field))
        .collect();

    let flatten_token = flatten_token((&data.fields).iter());
    // 从节点捕获值
    let node_fields = get_value_from_node_token((&data.fields).iter());
    // 从属性捕获值
    let attribute_fields = get_value_from_attribute_token((&data.fields).iter());
    // 从文本捕获值
    let text_fields = get_value_from_text_token((&data.fields).iter());

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
            let var_name =
                TokenStream::from_str(format!("f_{}", ident.to_string()).as_str()).unwrap();
            return quote! {
              #ident:#var_name,
            };
        })
        .collect();

    Ok(quote! {
      impl easy_xml::XmlDeserialize for #name{
        fn deserialize(element: &easy_xml::XmlElement) -> Result<Self, easy_xml::de::Error>
        where
            Self: Sized {

            //属性变量定义
            #var_declare_token

            #flatten_token

            //文本内容捕获
            #text_fields

            match element {
              easy_xml::XmlElement::Node(node) => {

                  #attribute_fields

                  #node_fields
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
    let var_name = field.ident.as_ref().unwrap();
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

    let var_name = TokenStream::from_str(format!("f_{}", var_name.to_string()).as_str()).unwrap();

    quote! {
      let mut #var_name:#var_type =  #var_value ;
    }
}

pub fn flatten_token(fields: syn::punctuated::Iter<Field>) -> TokenStream {
    let mut count = 0;
    let token: TokenStream = fields
        .filter(|f| {
            let attrs = Attributes::new(&f.attrs);
            return attrs.flatten == true;
        })
        .map(|f| {
            count += 1;
            let var_instance = get_var_instance(f);
            quote! {
              {
                #var_instance
              }
            }
        })
        .collect();

    if count == 0 {
        quote! {}
    } else {
        quote! {
          #token
        }
    }
}

// 从节点中捕获变量
pub fn get_value_from_node_token(fields: syn::punctuated::Iter<Field>) -> TokenStream {
    let mut count = 0;
    let token: TokenStream = fields
        .filter(|f| {
            let attrs = Attributes::new(&f.attrs);
            return attrs.attribute == false && attrs.text == false && attrs.flatten == false;
        })
        .map(|f| {
            count += 1;

            let var_name = f.ident.as_ref().unwrap();
            let attrs = Attributes::new(&f.attrs);
            let owned_name_match = owned_name_match(var_name, &attrs);
            let var_instance = get_var_instance(f);
            quote! {
              if #owned_name_match {
                #var_instance
              }
            }
        })
        .collect();

    if count == 0 {
        quote! {}
    } else {
        quote! {
          for element in &node.elements {
            match element {
              easy_xml::XmlElement::Node(node) => {
                  let name = &node.name;
                  #token
              }
              _ => {}
            }
          }
        }
    }
}

// 从属性值中捕获变量
pub fn get_value_from_attribute_token(fields: syn::punctuated::Iter<Field>) -> TokenStream {
    let mut count = 0;
    let token: TokenStream = fields
        .filter(|f| {
            let attrs = Attributes::new(&f.attrs);
            return attrs.attribute == true;
        })
        .map(|f| {
            count += 1;

            let var_name = f.ident.as_ref().unwrap();
            let attrs = Attributes::new(&f.attrs);
            let owned_name_match = owned_name_match(var_name, &attrs);
            let var_instance = get_var_instance(&f);
            quote! {
              if #owned_name_match {
                let element = easy_xml::XmlElement::Text(attr.value.clone());

                #var_instance
              }
            }
        })
        .collect();

    if count == 0 {
        quote! {}
    } else {
        quote! {
          for attr in &node.attributes {
            let name = &attr.name;
            #token
          }
        }
    }
}

// 从文本中捕获变量
pub fn get_value_from_text_token(fields: syn::punctuated::Iter<Field>) -> TokenStream {
    let mut count = 0;
    let token: TokenStream = fields
        .filter(|f| {
            let attrs = Attributes::new(&f.attrs);
            return attrs.attribute == false && attrs.text == true;
        })
        .map(|f| {
            count += 1;
            let var_instance = get_var_instance(&f);
            quote! {
              #var_instance
            }
        })
        .collect();

    if count == 0 {
        quote! {}
    } else {
        quote! {
          {
            let mut text = String::new();
            element.text(&mut text);
            let element = easy_xml::XmlElement::Text(text);
            #token
          }
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
    let token = TokenStream::from_str(ty.full_path().as_str()).unwrap();

    let var_name = field.ident.as_ref().unwrap();
    let var_name = TokenStream::from_str(format!("f_{}", var_name.to_string()).as_str()).unwrap();

    if is_vec {
        return quote! {
          #var_name.push(#token::deserialize(&element)?);
        };
    } else if ty.has_option() {
        return quote! {
          *#var_name = #token::deserialize(&element)?;
        };
    } else {
        quote! {
          *#var_name = Some(#token::deserialize(&element)?);
        }
    }
}

pub fn var_re_bind(field: &Field) -> TokenStream {
    let tw = TypeWapper::new(&field.ty);
    let var_name = field.ident.as_ref().unwrap();
    let var_name = TokenStream::from_str(format!("f_{}", var_name.to_string()).as_str()).unwrap();

    if tw.has_vec() {
        return quote! {};
    } else if tw.has_option() {
        return quote! {
          let #var_name = *#var_name;
        };
    }

    let msg = format!("Field {} has no value!", var_name);

    quote! {
      let #var_name = match *#var_name{
        Some(val) =>val,
        None =>  return Err(easy_xml::de::Error::Other(#msg.to_string())),
      };
    }
}

pub fn fields_check(fields: syn::punctuated::Iter<Field>) {
    for f in fields {
        let ty = TypeWapper::new(&f.ty);
        //Field type check
        ty.type_check();

        //Vec and text are mutually exclusive
        if ty.has_vec() {
            let attrs = Attributes::new(&f.attrs);
            if attrs.text {
                panic!("Vec and text are mutually exclusive!")
            }
        }

        let mut count = 0;
        let attrs = Attributes::new(&f.attrs);
        if attrs.text {
            count += 1;
        }
        if attrs.attribute {
            count += 1;
        }
        if attrs.flatten {
            count += 1;
        }
        if count > 1 {
            panic!("text, attribute and flatten are mutually exclusive!")
        }
    }
}
