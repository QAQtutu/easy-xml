use proc_macro2::{token_stream::IntoIter, Delimiter, Ident, TokenStream, TokenTree};
use quote::quote;
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
    pub rename: Option<String>, // pub namespaces: BTreeMap<String, String>,
}

impl Attributes {
    pub fn new(attrs: &Vec<Attribute>) -> Self {
        let mut attribute = false;
        let mut text = false;
        let mut flatten = false;
        let mut prefix = None;
        let mut rename = None;

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
    let tag = match &attrs.rename {
        Some(_) => attrs.rename.as_ref().unwrap().clone(),
        None => val_name.to_string(),
    };

    let prefix_match = match &attrs.prefix {
        Some(prefix) => {
            quote! {
              && match &name.prefix {
                Some(prefix) => prefix.as_str() == #prefix,
                None => false,
              }
            }
        }
        None => quote! {},
    };
    quote! {
      #tag == &name.local_name #prefix_match
    }
}
