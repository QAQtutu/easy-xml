use std::str::FromStr;

use proc_macro2::{token_stream::IntoIter, Delimiter, Ident, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{Attribute, Type};
pub struct TypeWapper {
    pub ty: Type,
}

impl TypeWapper {
    pub fn new(ty: &Type) -> TypeWapper {
        return TypeWapper { ty: ty.clone() };
    }

    pub fn get_ident(&self) -> Ident {
        match &self.ty {
            syn::Type::Path(ref path) => {
                for i in &path.path.segments {
                    return i.ident.clone();
                }
            }
            _ => {
                panic!("");
            }
        };
        panic!("");
    }

    pub fn full_path(&self) -> String {
        match &self.ty {
            syn::Type::Path(ref path) => {
                let paths = &path.path.segments;

                let mut full_path = String::new();

                let mut i = 0;
                for p in paths.iter() {
                    full_path.push_str(p.ident.to_string().as_str());
                    if i < paths.len() - 1 {
                        full_path.push_str("::");
                    }
                    i += 1;
                }

                return full_path;
            }
            _ => {
                panic!("");
            }
        };
    }

    pub fn next_type(&self) -> Option<Self> {
        match &self.ty {
            syn::Type::Path(ref path) => {
                for i in &path.path.segments {
                    match &i.arguments {
                        syn::PathArguments::None => return None,
                        syn::PathArguments::AngleBracketed(arguments) => {
                            for argument in &arguments.args {
                                match argument {
                                    syn::GenericArgument::Type(t) => {
                                        return Some(TypeWapper::new(t));
                                    }
                                    _ => {
                                        panic!("");
                                    }
                                }
                            }
                            return None;
                        }
                        syn::PathArguments::Parenthesized(_) => todo!(),
                    }
                }
            }
            _ => {
                panic!("");
            }
        };
        panic!("");
    }

    pub fn has_vec(&self) -> bool {
        return self.has_subtype("Vec");
    }

    pub fn has_option(&self) -> bool {
        return self.has_subtype("Option");
    }

    pub fn subtype_count(&self, subtype: &str) -> usize {
        let mut count = 0;
        self.type_for_each(|ty| {
            if ty.get_ident().to_string().as_str() == subtype {
                count += 1;
            }
        });
        return count;
    }

    pub fn has_subtype(&self, subtype: &str) -> bool {
        return self.subtype_count(subtype) > 0;
    }

    pub fn type_check(&self) {
        let mut vec = 0;
        let mut option = 0;

        let mut path = String::new();

        self.type_for_each(|ty| {
            let ident = ty.get_ident().to_string();

            path += ident.to_string().as_str();
            path += ">";

            match ident.as_str() {
                "Option" => option += 1,
                "Vec" => vec += 1,
                _ => {}
            }
        });

        if vec >= 2 {
            panic!("Multi level nesting of Vec is not supported({})", path);
        }
        if option >= 2 {
            panic!("Multi level nesting of Option is not supported({})", path);
        }
        if option == 1 && self.get_ident().to_string().as_str() != "Option" {
            panic!("Option must be at the first level({})", path);
        }
        if vec == 1 && self.get_ident().to_string().as_str() != "Vec" {
            panic!("Vec must be at the first level({})", path);
        }
    }

    // 子类型遍历
    // Foreach Subtype
    pub fn type_for_each<F>(&self, mut f: F)
    where
        F: FnMut(&Self),
    {
        f(self);

        let mut next = self.next_type();
        loop {
            match next.as_ref() {
                Some(ty) => {
                    f(&ty);
                    next = ty.next_type();
                }
                None => break,
            }
        }
    }
}

#[derive(Debug)]
pub struct Attributes {
    pub flatten: bool,
    pub text: bool,
    pub attribute: bool,
    pub prefix: Option<String>,
    pub rename: Option<String>,
    // pub namespaces: BTreeMap<String, String>,
    pub enums: bool,
}

impl Attributes {
    pub fn new(attrs: &Vec<Attribute>) -> Self {
        let mut attribute = false;
        let mut text = false;
        let mut flatten = false;
        let mut prefix = None;
        let mut rename = None;
        let mut enums = false;

        for attr in attrs.iter().filter(|a| a.path.is_ident("easy_xml")) {
            let mut attr_iter = attr.clone().tokens.into_iter();
            if let Some(TokenTree::Group(group)) = attr_iter.next() {
                if group.delimiter() == Delimiter::Parenthesis {
                    let mut attr_iter = group.stream().into_iter();
                    while let Some(item) = attr_iter.next() {
                        if let TokenTree::Ident(ident) = item {
                            match ident.to_string().as_str() {
                                "attribute" => {
                                    attribute = true;
                                }
                                "text" => {
                                    text = true;
                                }
                                "flatten" => {
                                    flatten = true;
                                }
                                "enum" => {
                                    enums = true;
                                }
                                "prefix" => {
                                    prefix = get_value(&mut attr_iter);
                                }
                                "rename" => {
                                    rename = get_value(&mut attr_iter);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Attributes {
            flatten,
            text,
            attribute,
            prefix,
            rename,
            enums,
        }
    }
}

fn get_value(iter: &mut IntoIter) -> Option<String> {
    if let (Some(TokenTree::Punct(operator)), Some(TokenTree::Literal(value))) =
        (iter.next(), iter.next())
    {
        if operator.as_char() == '=' {
            Some(value.to_string().replace("\"", ""))
        } else {
            None
        }
    } else {
        None
    }
}

// OwnedName匹配
pub fn owned_name_match(val_name: &Ident, attrs: &Attributes) -> TokenStream {
    let mut token = String::from("true ");
    match &attrs.rename {
        Some(rename) => {
            if attrs.enums {
                token.push_str("&& { true");
                for i in rename.split("|") {
                    token.push_str(format!("|| \"{}\" == &name.local_name ", i).as_str())
                }
                token.push_str(" }");
            } else {
                token.push_str(format!("&& \"{}\" == &name.local_name", rename).as_str())
            }
        }
        None => token.push_str(format!("&& \"{}\" == &name.local_name ", val_name).as_str()),
    }

    match &attrs.prefix {
        Some(prefix) => {
            token.push_str(format!("{{ && match &name.prefix {{ Some(prefix) => prefix.as_str() == \"{}\", None => false, }}", prefix).as_str());
        }
        None => {}
    };

    let token = <TokenStream as std::str::FromStr>::from_str(token.as_str()).unwrap();

    quote! {
      #token
    }
}

pub struct Field<'a> {
    field: &'a syn::Field,
    ty: TypeWapper,
    pub attrs: Attributes,
    index: i32,
}
impl<'a> Field<'a> {
    pub fn _from_named(field: &'a syn::Field) -> Self {
        Field {
            field,
            index: -1,
            attrs: Attributes::new(&field.attrs),
            ty: TypeWapper::new(&field.ty),
        }
    }
    pub fn from_unnamed(field: &'a syn::Field, index: i32) -> Self {
        Field {
            field,
            index,
            attrs: Attributes::new(&field.attrs),
            ty: TypeWapper::new(&field.ty),
        }
    }
    pub fn var_name(&self) -> TokenStream {
        match self.field.ident.as_ref() {
            Some(i) => TokenStream::from_str(format!("f_{}", i.to_string()).as_str()).unwrap(),
            None => TokenStream::from_str(format!("f_{}", self.index).as_str()).unwrap(),
        }
    }

    pub fn owned_name_match(&self) -> TokenStream {
        let mut token = String::from("true ");
        let attrs = &self.attrs;
        match &attrs.rename {
            Some(rename) => {
                if attrs.enums {
                    token.push_str("&& { true");
                    for i in rename.split("|") {
                        token.push_str(format!("|| \"{}\" == &name.local_name ", i).as_str())
                    }
                    token.push_str(" }");
                } else {
                    token.push_str(format!("&& \"{}\" == &name.local_name", rename).as_str())
                }
            }
            None => match self.field.ident.as_ref() {
                Some(ident) => {
                    token.push_str(format!("&& \"{}\" == &name.local_name ", ident).as_str())
                }
                None => panic!("Unnamed need rename!"),
            },
        }

        match &attrs.prefix {
            Some(prefix) => {
                token.push_str(format!("{{ && match &name.prefix {{ Some(prefix) => prefix.as_str() == \"{}\", None => false, }}", prefix).as_str());
            }
            None => {}
        };

        let token = <TokenStream as std::str::FromStr>::from_str(token.as_str()).unwrap();

        quote! {
          #token
        }
    }

    pub fn var_declare(&self) -> TokenStream {
        let var_name = self.var_name();

        let ty = &self.ty;
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
          let mut #var_name:#var_type =  #var_value ;
        }
    }

    pub fn get_var_instance(&self) -> TokenStream {
        let ty = TypeWapper::new(&self.field.ty);

        let is_vec = ty.has_vec();

        let ty = {
            if is_vec {
                ty.next_type().unwrap()
            } else {
                ty
            }
        };
        let token = TokenStream::from_str(ty.full_path().as_str()).unwrap();

        let var_name = self.var_name();

        if is_vec {
            return quote! {
              // let field___val : #ty_token =  ;
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
}

pub fn build_code_for_declare(fields: &Vec<Field>) -> TokenStream {
    fields.into_iter().map(|f| f.var_declare()).collect()
}
pub fn build_code_for_text(fields: &Vec<Field>) -> TokenStream {
    let text_code: TokenStream = (&fields)
        .into_iter()
        .filter(|f| f.attrs.text)
        .map(|f| f.get_var_instance())
        .collect();

    let text_code = quote! {
        {
          let mut text = String::new();
          element.text(&mut text);
          let element = easy_xml::XmlElement::Text(text);
          #text_code
        }
    };
    return text_code;
}

pub fn build_code_for_flatten(fields: &Vec<Field>) -> TokenStream {
    let flatten_code: TokenStream = (&fields)
        .into_iter()
        .filter(|f| f.attrs.flatten)
        .map(|f| f.get_var_instance())
        .collect();

    let flatten_code = quote! {
        {
          #flatten_code
        }
    };
    return flatten_code;
}

pub fn build_code_for_attribute(fields: &Vec<Field>) -> TokenStream {
    let mut count = 0;
    let attribute_code: TokenStream = (&fields)
        .into_iter()
        .filter(|f| f.attrs.attribute)
        .map(|f| {
            count += 1;
            let owned_name_match = f.owned_name_match();
            let var_instance = f.get_var_instance();
            quote! {
              if #owned_name_match {
                let element = easy_xml::XmlElement::Text(attr.value.clone());
                #var_instance
              }
            }
        })
        .collect();

    if count > 0 {
        quote! {
          for attr in &node.attributes {
            let name = &attr.name;
            #attribute_code
          }
        }
    } else {
        quote! {}
    }
}

pub fn build_code_for_children(fields: &Vec<Field>) -> TokenStream {
    let mut count = 0;
    let code: TokenStream = (&fields)
        .into_iter()
        .filter(|f| f.attrs.attribute == false && f.attrs.text == false && f.attrs.flatten == false)
        .map(|f| {
            count += 1;
            let owned_name_match = f.owned_name_match();
            let var_instance = f.get_var_instance();
            quote! {
              if #owned_name_match {
                #var_instance
              }
            }
        })
        .collect();

    if count > 0 {
        quote! {
          for element in &node.elements {
            match element {
              easy_xml::XmlElement::Node(node) => {
                  let name = &node.name;
                  #code
              }
              _ => {}
            }
          }
        }
    } else {
        quote! {}
    }
}

pub fn var_rebind(fields: &Vec<Field>) -> TokenStream {
    (&fields)
        .into_iter()
        .map(|f| {
            let var_name = f.var_name();
            if f.ty.has_vec() {
                quote! {}
            } else if f.ty.has_option() {
                quote! {
                  let #var_name = *#var_name;
                }
            } else {
                quote! {
                  let #var_name = easy_xml::de::unwrap_option(*#var_name)?;
                }
            }
        })
        .collect()
}

pub fn var_collect(fields: &Vec<Field>) -> TokenStream {
    (&fields)
        .into_iter()
        .map(|f| {
            let var_name = f.var_name();
            match &f.field.ident {
                Some(ident) => quote! {#ident:#var_name,},
                None => quote! {#var_name,},
            }
        })
        .collect()
}
