use std::io::Read;

use crate::{XmlDeserialize, XmlDocument, XmlElement, XmlNode};
use xml::reader::{EventReader, XmlEvent};

impl XmlDocument {
    pub fn from_str(xml: &str) -> xml::reader::Result<XmlDocument> {
        let reader = EventReader::new(xml.as_bytes());
        return XmlDocument::from_read(reader);
    }
    pub fn from_read<R: Read>(reader: EventReader<R>) -> xml::reader::Result<XmlDocument> {
        let mut doc = None;

        let mut stack = Vec::new();
        for e in reader {
            let e = e?;
            match e {
                XmlEvent::StartDocument {
                    version,
                    encoding,
                    standalone,
                } => {
                    doc = Some(XmlDocument {
                        version,
                        encoding,
                        standalone,
                        elements: vec![],
                    });
                }
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    let node = XmlNode {
                        name,
                        attributes,
                        namespace,
                        elements: vec![],
                    };
                    stack.push(XmlElement::Node(node));
                }
                XmlEvent::EndElement { name: _ } => {
                    let node = stack.pop().unwrap();
                    add_element_to_parent(node, &mut stack, (&mut doc).as_mut().unwrap());
                }
                XmlEvent::Characters(s) => {
                    add_element_to_parent(
                        XmlElement::Text(s),
                        &mut stack,
                        (&mut doc).as_mut().unwrap(),
                    );
                }
                XmlEvent::Comment(s) => {
                    add_element_to_parent(
                        XmlElement::Comment(s),
                        &mut stack,
                        (&mut doc).as_mut().unwrap(),
                    );
                }
                XmlEvent::CData(s) => {
                    add_element_to_parent(
                        XmlElement::CData(s),
                        &mut stack,
                        (&mut doc).as_mut().unwrap(),
                    );
                }
                XmlEvent::Whitespace(s) => {
                    add_element_to_parent(
                        XmlElement::Whitespace(s),
                        &mut stack,
                        (&mut doc).as_mut().unwrap(),
                    );
                }
                _ => {}
            }
        }

        return Ok(doc.unwrap());
    }
}

fn add_element_to_parent(node: XmlElement, stack: &mut Vec<XmlElement>, doc: &mut XmlDocument) {
    if stack.len() > 0 {
        let idx = stack.len() - 1;
        let parent = stack.get_mut(idx).unwrap();
        match parent {
            XmlElement::Node(n) => {
                n.elements.push(node);
            }
            _ => {}
        }
    } else {
        doc.elements.push(node);
    }
}

pub fn from_str<T: XmlDeserialize>(xml: &str) -> Result<T, Error> {
    let doc = match XmlDocument::from_str(xml) {
        Ok(doc) => doc,
        Err(_) => return Err(Error::BadXml),
    };
    match doc.elements.get(0) {
        Some(root) => T::deserialize(root),
        None => return Err(Error::BadXml),
    }
}

#[derive(Debug)]
pub enum Error {
    XmlError(xml::reader::Error),
    BadXml,
    Other(String),
}

#[inline(always)]
pub fn unwrap_option<T>(op: Option<T>) -> Result<T, Error> {
    match op {
        Some(val) => return Ok(val),
        None => {
            return Err(Error::Other("Failed to get value!".to_string()));
        }
    }
}
