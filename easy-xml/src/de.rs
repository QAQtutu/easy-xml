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
    let doc = XmlDocument::from_str(xml).unwrap();
    let i = doc.elements.get(0).unwrap();
    return T::deserialize(i);
}

#[derive(Debug)]
pub enum Error {
    XmlError(xml::reader::Error),
    Other(String),
}
