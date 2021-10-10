use xml::{attribute::OwnedAttribute, name::OwnedName, namespace::Namespace};

use crate::{XmlDocument, XmlElement};

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

struct SerializeContext {
    xml: String,
    deep: u32,
    only_one_child: bool,
    root: bool,
    settings: SerializeSettings,
}

#[inline]
fn write_indent(ctx: &mut SerializeContext) {
    if ctx.settings.pretty_format {
        for _i in 0..(ctx.settings.indent * ctx.deep) {
            ctx.xml.push_str(" ");
        }
    }
}

#[inline]
fn write_name(ctx: &mut SerializeContext, name: &OwnedName) {
    if let Some(prefix) = &name.prefix {
        ctx.xml.push_str(prefix.as_str());
        ctx.xml.push_str(":");
    }
    ctx.xml.push_str(name.local_name.as_str());
}

fn write_namespace(ctx: &mut SerializeContext, namespace: &Namespace) {
    if !ctx.root {
        return;
    }
    for i in namespace {
        let key = i.0.trim();
        if key == "" || key == "xml" || key == "xmlns" {
            continue;
        }
        ctx.xml.push_str(" xmlns:");
        ctx.xml.push_str(i.0);
        ctx.xml.push_str("=\"");
        ctx.xml.push_str(i.1);
        ctx.xml.push_str("\"");
    }
    ctx.root = false;
}

#[inline]
fn write_attributes(ctx: &mut SerializeContext, attributes: &Vec<OwnedAttribute>) {
    for attribute in attributes.iter() {
        ctx.xml.push_str(" ");
        write_name(ctx, &attribute.name);
        ctx.xml.push_str("=\"");
        ctx.xml.push_str(attribute.value.as_str());
        ctx.xml.push_str("\"");
    }
}
#[inline]
fn write_children(ctx: &mut SerializeContext, elements: &Vec<XmlElement>) {
    if elements.len() == 0 {
        return;
    };

    ctx.only_one_child = elements.len() == 1;

    let only_text_child = ctx.only_one_child && {
        match elements.get(0).unwrap() {
            XmlElement::Text(_) => true,
            _ => false,
        }
    };

    ctx.deep += 1;
    for e in elements {
        e.serialize(ctx);
    }
    ctx.deep -= 1;

    if !only_text_child {
        write_line_break(ctx);
        write_indent(ctx);
    }
}
#[inline]
fn write_line_break(ctx: &mut SerializeContext) {
    if ctx.settings.pretty_format {
        ctx.xml.push_str("\n");
    }
}

impl XmlElement {
    fn serialize(&self, ctx: &mut SerializeContext) {
        match self {
            XmlElement::Text(str) => {
                if !ctx.only_one_child {
                    write_line_break(ctx);
                    write_indent(ctx);
                }
                ctx.xml.push_str(str.trim());
            }
            XmlElement::Node(node) => {
                let node = node.borrow_mut();
                write_line_break(ctx);
                write_indent(ctx);

                ctx.xml.push_str("<");
                write_name(ctx, &node.name);

                write_namespace(ctx, &node.namespace);

                write_attributes(ctx, &node.attributes);
                ctx.xml.push_str(">");

                write_children(ctx, &node.elements);

                ctx.xml.push_str("</");
                write_name(ctx, &node.name);
                ctx.xml.push_str(">");
            }
            XmlElement::Whitespace(_) => {}
            XmlElement::Comment(_) => {}
            XmlElement::CData(_) => {}
        }
    }
}

impl XmlDocument {
    pub fn serialize(&self) -> String {
        let settings = SerializeSettings::default();
        return self.serialize_with_settings(settings);
    }
    pub fn serialize_with_settings(&self, settings: SerializeSettings) -> String {
        let mut ctx = SerializeContext {
            deep: 0,
            settings: settings,
            only_one_child: false,
            root: true,
            xml: String::new(),
        };

        ctx.xml.push_str(
            format!(
                "<?xml version=\"{}\" encoding=\"{}\"{}?>",
                self.version,
                self.encoding,
                match self.standalone {
                    Some(b) => {
                        if b {
                            " standalone=\"yes\""
                        } else {
                            " standalone=\"no\""
                        }
                    }
                    None => "",
                }
            )
            .as_str(),
        );
        for e in &self.elements {
            ctx.root = true;
            e.serialize(&mut ctx);
        }
        return ctx.xml;
    }
}
