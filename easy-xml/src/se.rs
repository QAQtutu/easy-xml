use std::{borrow::Cow, cell::RefCell, io::Write, rc::Rc, string::FromUtf8Error};

use xml::{
    attribute::Attribute,
    name::{Name, OwnedName},
    writer::XmlEvent,
    EmitterConfig, EventWriter,
};

use crate::{XmlDocument, XmlElement, XmlNode, XmlSerialize};

pub struct SerializeSettings {
    pub indent: u32,
    pub pretty_format: bool,
}

impl Default for SerializeSettings {
    fn default() -> Self {
        Self {
            indent: 4,
            pretty_format: false,
        }
    }
}

fn owned_name_to_name(owned_name: &OwnedName) -> Name {
    Name {
        local_name: owned_name.local_name.as_str(),
        namespace: match &owned_name.namespace {
            Some(namespace) => Some(namespace.as_str()),
            None => None,
        },
        prefix: match &owned_name.prefix {
            Some(prefix) => Some(prefix.as_str()),
            None => None,
        },
    }
}

fn format_xml_element<W: Write>(
    w: &mut EventWriter<W>,
    element: &XmlElement,
) -> xml::writer::Result<()> {
    match element {
        XmlElement::Text(text) => {
            w.write(XmlEvent::characters(text.as_str()))?;
        }
        XmlElement::Node(node) => {
            let node = &*node.borrow_mut();
            let attributes = &node.attributes;
            let attributes = attributes
                .into_iter()
                .map(|attr| Attribute {
                    name: owned_name_to_name(&attr.name),
                    value: attr.value.as_str(),
                })
                .collect::<Vec<_>>();

            w.write(XmlEvent::StartElement {
                name: owned_name_to_name(&node.name),
                attributes: Cow::Borrowed(attributes.as_slice()),
                namespace: Cow::Borrowed(&node.namespace),
            })?;

            let elements = &node.elements;
            for e in elements {
                format_xml_element(w, e)?;
            }

            w.write(XmlEvent::EndElement {
                name: Some(owned_name_to_name(&node.name)),
            })?;
        }
        XmlElement::Whitespace(_) => {}
        XmlElement::Comment(comment) => {
            w.write(XmlEvent::Comment(comment))?;
        }
        XmlElement::CData(cdata) => {
            w.write(XmlEvent::CData(cdata))?;
        }
    }
    Ok(())
}
fn format_xml<W: Write>(w: &mut EventWriter<W>, doc: &XmlDocument) -> xml::writer::Result<()> {
    w.write(XmlEvent::StartDocument {
        version: doc.version,
        encoding: Some(doc.encoding.as_str()),
        standalone: doc.standalone.clone(),
    })?;

    for e in &doc.elements {
        format_xml_element(w, e)?;
    }

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    EmitterError(xml::writer::Error),
    FromUtf8Error(FromUtf8Error),
}

pub fn to_string<T: XmlSerialize>(t: &T) -> Result<String, Error> {
    match to_bytes(t, "UTF-8") {
        Ok(v8) => match String::from_utf8(v8) {
            Ok(s) => return Ok(s),
            Err(e) => return Err(Error::FromUtf8Error(e)),
        },
        Err(e) => return Err(Error::EmitterError(e)),
    }
}

pub fn to_bytes<T: XmlSerialize>(t: &T, encoding: &str) -> xml::writer::Result<Vec<u8>> {
    let mut v8: Vec<u8> = Vec::new();
    let mut writer = EmitterConfig::new().create_writer(&mut v8);
    serialize(t, &mut writer, encoding)?;
    return Ok(v8);
}

fn serialize<T: XmlSerialize, W: Write>(
    t: &T,
    writer: &mut EventWriter<W>,
    encoding: &str,
) -> xml::writer::Result<()> {
    let mut doc = XmlDocument {
        version: xml::common::XmlVersion::Version10,
        encoding: encoding.to_string(),
        standalone: None,
        elements: Vec::new(),
    };

    let mut root = XmlElement::Node(Rc::new(RefCell::new(XmlNode::empty())));
    t.serialize(&mut root);

    doc.elements.push(root);

    format_xml(writer, &doc)
}
