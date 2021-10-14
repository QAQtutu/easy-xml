extern crate easy_xml_derive;

use std::{
    borrow::Borrow,
    cell::RefCell,
    rc::{Rc, Weak},
};

use xml::{attribute::OwnedAttribute, common::XmlVersion, name::OwnedName, namespace::Namespace};

#[derive(Debug, Clone)]
pub struct XmlDocument {
    pub version: XmlVersion,
    pub encoding: String,
    pub standalone: Option<bool>,
    pub elements: Vec<XmlElement>,
}

#[derive(Debug, Clone)]
pub enum XmlElement {
    Text(String),
    Node(Rc<RefCell<XmlNode>>),
    Whitespace(String),
    Comment(String),
    CData(String),
}

#[derive(Debug, Clone)]
pub struct XmlNode {
    pub name: OwnedName,
    pub attributes: Vec<OwnedAttribute>,
    pub namespace: Namespace,
    pub elements: Vec<XmlElement>,
    pub parent: Option<Weak<RefCell<XmlNode>>>,
}

pub trait XmlDeserialize {
    fn deserialize(element: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized;
}
pub trait XmlSerialize {
    fn serialize(&self, element: &mut XmlElement)
    where
        Self: Sized;
}

pub mod de;
pub mod se;

impl XmlNode {
    pub fn text(&self, string: &mut String) {
        for e in &self.elements {
            e.text(string);
        }
    }

    pub fn empty() -> Self {
        XmlNode {
            name: OwnedName {
                local_name: String::new(),
                namespace: None,
                prefix: None,
            },
            attributes: Vec::new(),
            namespace: Namespace::empty(),
            elements: Vec::new(),
            parent: None,
        }
    }
}
impl XmlElement {
    pub fn text(&self, string: &mut String) {
        match self {
            XmlElement::Text(text) => string.push_str(text.as_str()),
            XmlElement::Node(node) => {
                let node = node.as_ref().borrow();
                node.text(string);
            }
            XmlElement::Whitespace(_) => {}
            XmlElement::Comment(_) => {}
            XmlElement::CData(_) => {}
        }
    }
}

impl<T: XmlDeserialize> XmlDeserialize for Option<T>
where
    T: Sized,
{
    fn deserialize(element: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(element) {
            Ok(obj) => Ok(Some(obj)),
            Err(e) => Err(e),
        }
    }
}

impl<T: XmlDeserialize> XmlDeserialize for Box<T>
where
    T: Sized,
{
    fn deserialize(element: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(element) {
            Ok(obj) => Ok(Box::new(obj)),
            Err(e) => Err(e),
        }
    }
}

impl<T: XmlDeserialize> XmlDeserialize for std::rc::Rc<T>
where
    T: Sized,
{
    fn deserialize(element: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(element) {
            Ok(obj) => Ok(std::rc::Rc::new(obj)),
            Err(e) => Err(e),
        }
    }
}

impl<T: XmlDeserialize> XmlDeserialize for std::sync::Arc<T>
where
    T: Sized,
{
    fn deserialize(element: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(element) {
            Ok(obj) => Ok(std::sync::Arc::new(obj)),
            Err(e) => Err(e),
        }
    }
}

impl<T: XmlDeserialize> XmlDeserialize for std::cell::Cell<T>
where
    T: Sized,
{
    fn deserialize(element: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(element) {
            Ok(obj) => Ok(std::cell::Cell::new(obj)),
            Err(e) => Err(e),
        }
    }
}

impl<T: XmlDeserialize> XmlDeserialize for std::cell::RefCell<T>
where
    T: Sized,
{
    fn deserialize(element: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(element) {
            Ok(obj) => Ok(std::cell::RefCell::new(obj)),
            Err(e) => Err(e),
        }
    }
}

impl XmlDeserialize for String {
    fn deserialize(element: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        let mut text = String::new();
        element.text(&mut text);
        Ok(text)
    }
}

macro_rules! impl_de_for_number {
    ($x:ty) => {
        impl XmlDeserialize for $x {
            fn deserialize(element: &XmlElement) -> Result<Self, de::Error>
            where
                Self: Sized,
            {
                let str = String::deserialize(element)?;
                let str = str.trim();
                match str.parse::<$x>() {
                    Ok(val) => Ok(val),
                    Err(_) => {
                        let msg = format!("\"{}\" can not convert to number!", str);

                        Err(de::Error::Other(msg))
                    }
                }
            }
        }
    };
}

impl_de_for_number!(usize);
impl_de_for_number!(isize);

impl_de_for_number!(u8);
impl_de_for_number!(u16);
impl_de_for_number!(u32);
impl_de_for_number!(u64);
impl_de_for_number!(u128);

impl_de_for_number!(i8);
impl_de_for_number!(i16);
impl_de_for_number!(i32);
impl_de_for_number!(i64);
impl_de_for_number!(i128);

impl_de_for_number!(f32);
impl_de_for_number!(f64);

// --------------------------------------------------------------------------------------------------------------------

impl XmlSerialize for String {
    fn serialize(&self, node: &mut XmlElement)
    where
        Self: Sized,
    {
        match node {
            XmlElement::Text(text) => {
                text.push_str(self.as_str());
            }
            XmlElement::Node(node) => {
                node.as_ref()
                    .borrow_mut()
                    .elements
                    .push(XmlElement::Text(self.clone()));
            }
            _ => {}
        }
    }
}

impl<T: XmlSerialize> XmlSerialize for Option<T> {
    fn serialize(&self, element: &mut XmlElement)
    where
        Self: Sized,
    {
        match self {
            Some(t) => {
                t.serialize(element);
            }
            None => {}
        }
    }
}

impl<T: XmlSerialize> XmlSerialize for Box<T> {
    fn serialize(&self, element: &mut XmlElement)
    where
        Self: Sized,
    {
        self.as_ref().serialize(element);
    }
}
impl<T: XmlSerialize> XmlSerialize for Rc<T> {
    fn serialize(&self, element: &mut XmlElement)
    where
        Self: Sized,
    {
        self.as_ref().serialize(element);
    }
}
impl<T: XmlSerialize> XmlSerialize for std::sync::Arc<T> {
    fn serialize(&self, element: &mut XmlElement)
    where
        Self: Sized,
    {
        self.as_ref().serialize(element);
    }
}

impl<T: XmlSerialize> XmlSerialize for std::cell::Cell<T> {
    fn serialize(&self, element: &mut XmlElement)
    where
        Self: Sized,
    {
        self.borrow().serialize(element);
    }
}

impl<T: XmlSerialize> XmlSerialize for std::cell::RefCell<T> {
    fn serialize(&self, element: &mut XmlElement)
    where
        Self: Sized,
    {
        self.borrow().serialize(element);
    }
}

macro_rules! impl_se_for_number {
    ($x:ty) => {
        impl XmlSerialize for $x {
            fn serialize(&self, element: &mut XmlElement)
            where
                Self: Sized,
            {
                match element {
                    XmlElement::Text(text) => {
                        text.push_str(self.to_string().as_str());
                    }
                    XmlElement::Node(node) => {
                        node.as_ref()
                            .borrow_mut()
                            .elements
                            .push(XmlElement::Text(self.to_string()));
                    }
                    _ => {}
                }
            }
        }
    };
}

impl_se_for_number!(usize);
impl_se_for_number!(isize);

impl_se_for_number!(u8);
impl_se_for_number!(u16);
impl_se_for_number!(u32);
impl_se_for_number!(u64);
impl_se_for_number!(u128);

impl_se_for_number!(i8);
impl_se_for_number!(i16);
impl_se_for_number!(i32);
impl_se_for_number!(i64);
impl_se_for_number!(i128);

impl_se_for_number!(f32);
impl_se_for_number!(f64);
