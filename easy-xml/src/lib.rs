extern crate easy_xml_derive;

use xml::{attribute::OwnedAttribute, common::XmlVersion, name::OwnedName, namespace::Namespace};

#[derive(Debug)]
pub struct XmlDocument {
    pub version: XmlVersion,
    pub encoding: String,
    pub standalone: Option<bool>,
    pub elements: Vec<XmlElement>,
}

#[derive(Debug)]
pub enum XmlElement {
    Text(String),
    Node(XmlNode),
    Whitespace(String),
    Comment(String),
    CData(String),
}
#[derive(Debug)]
pub struct XmlNode {
    pub name: OwnedName,
    pub attributes: Vec<OwnedAttribute>,
    pub namespace: Namespace,
    pub elements: Vec<XmlElement>,
}

pub trait XmlDeserialize {
    fn deserialize(node: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized;
}
pub trait XmlSerialize {}

pub mod de;
pub mod se;

impl XmlNode {
    pub fn text(&self, string: &mut String) {
        for e in &self.elements {
            e.text(string);
        }
    }
}
impl XmlElement {
    pub fn text(&self, string: &mut String) {
        match self {
            XmlElement::Text(text) => string.push_str(text.as_str()),
            XmlElement::Node(node) => {
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
    fn deserialize(node: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(node) {
            Ok(obj) => Ok(Some(obj)),
            Err(e) => Err(e),
        }
    }
}

impl<T: XmlDeserialize> XmlDeserialize for Box<T>
where
    T: Sized,
{
    fn deserialize(node: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(node) {
            Ok(obj) => Ok(Box::new(obj)),
            Err(e) => Err(e),
        }
    }
}

impl<T: XmlDeserialize> XmlDeserialize for std::rc::Rc<T>
where
    T: Sized,
{
    fn deserialize(node: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(node) {
            Ok(obj) => Ok(std::rc::Rc::new(obj)),
            Err(e) => Err(e),
        }
    }
}

impl<T: XmlDeserialize> XmlDeserialize for std::sync::Arc<T>
where
    T: Sized,
{
    fn deserialize(node: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(node) {
            Ok(obj) => Ok(std::sync::Arc::new(obj)),
            Err(e) => Err(e),
        }
    }
}

impl<T: XmlDeserialize> XmlDeserialize for std::cell::Cell<T>
where
    T: Sized,
{
    fn deserialize(node: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(node) {
            Ok(obj) => Ok(std::cell::Cell::new(obj)),
            Err(e) => Err(e),
        }
    }
}

impl<T: XmlDeserialize> XmlDeserialize for std::cell::RefCell<T>
where
    T: Sized,
{
    fn deserialize(node: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        match T::deserialize(node) {
            Ok(obj) => Ok(std::cell::RefCell::new(obj)),
            Err(e) => Err(e),
        }
    }
}

impl XmlDeserialize for String {
    fn deserialize(node: &XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        let mut text = String::new();
        node.text(&mut text);
        Ok(text)
    }
}

macro_rules! impl_de_for_number {
    ($x:ty) => {
        impl XmlDeserialize for $x {
            fn deserialize(node: &XmlElement) -> Result<Self, de::Error>
            where
                Self: Sized,
            {
                let str = String::deserialize(node)?;
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
