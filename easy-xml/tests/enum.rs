use easy_xml::{de, se};

#[macro_use]
extern crate easy_xml_derive;

#[test]
fn test() {
    {
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        #[easy_xml(root)]
        struct Content {
            #[easy_xml(rename = "Unnamed|Named|Unit", enum)]
            enums: Vec<EnumTest>,
        }
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        struct Flatten {
            #[easy_xml(attribute)]
            attr: String,
            #[easy_xml(text)]
            text: String,
        }
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        enum EnumTest {
            Unnamed(
                #[easy_xml(attribute, rename = "attr")] String,
                #[easy_xml(text)] String,
                #[easy_xml(rename = "Node")] String,
                #[easy_xml(flatten)] Flatten,
            ),
            Named {
                #[easy_xml(attribute)]
                attr: String,
                #[easy_xml(text)]
                text: String,
                #[easy_xml(rename = "Node")]
                node: String,
                #[easy_xml(flatten)]
                flatten: Flatten,
            },
            Unit,
        }

        let xml = r#"
          <Content>
            <Unnamed attr="value">
              <Node>node</Node>
            </Unnamed>
            <Named attr="value1">
              <Node>node1</Node>
            </Named>
            <Unit/>
          </Content>
        "#;

        let content: Content = easy_xml::de::from_str(xml).unwrap();

        assert_eq!(content.enums.len(), 3);

        for e in &content.enums {
            match e {
                EnumTest::Unnamed(attr, text, node, flatten) => {
                    assert_eq!(attr.as_str(), "value");
                    assert_eq!(text.as_str(), "node");
                    assert_eq!(node.as_str(), "node");
                    assert_eq!(flatten.attr.as_str(), "value");
                    assert_eq!(flatten.text.as_str(), "node");
                }
                EnumTest::Named {
                    attr,
                    text,
                    node,
                    flatten,
                } => {
                    assert_eq!(attr.as_str(), "value1");
                    assert_eq!(text.as_str(), "node1");
                    assert_eq!(node.as_str(), "node1");
                    assert_eq!(flatten.attr.as_str(), "value1");
                    assert_eq!(flatten.text.as_str(), "node1");
                }
                EnumTest::Unit => {}
            }
        }

        let xml = se::to_string(&content).unwrap();

        println!("{}", xml);

        assert_eq!(
            xml.as_str(),
            r#"<?xml version="1.0" encoding="UTF-8"?><Content><Unnamed attr="value" attr="value">node<Node>node</Node>node</Unnamed><Named attr="value1" attr="value1">node1<Node>node1</Node>node1</Named><Unit /></Content>"#
        );
    }
}

#[test]

// rename捕获多种Tag，to_text将捕获的tag内容转成Text类型传递
fn test_for_node() {
    #[derive(PartialEq, Debug, XmlDeserialize, XmlSerialize)]
    enum Type {
        T1,
        T2,
        T3,
    }
    #[derive(PartialEq, Debug, XmlDeserialize, XmlSerialize)]
    enum Obj {
        Text,
        Img,
        Video,
    }
    #[derive(PartialEq, Debug, XmlDeserialize, XmlSerialize)]
    struct Node {
        #[easy_xml(rename = "Type", to_text)]
        ty: Type,
        #[easy_xml(rename = "Text|Img|Video")]
        objs: Vec<Obj>,
    }

    let node = Node {
        ty: Type::T2,
        objs: vec![Obj::Text, Obj::Img, Obj::Video],
    };
    let xml = se::to_string(&node).unwrap();

    assert_eq!(
        xml.as_str(),
        r#"<?xml version="1.0" encoding="UTF-8"?><Node><Type>T2</Type><Text /><Img /><Video /></Node>"#
    );

    assert_eq!(node, de::from_str::<Node>(xml.as_str()).unwrap());
}

#[test]
fn test_enum_with_rename() {
    #[derive(PartialEq, Debug, XmlDeserialize, XmlSerialize)]
    enum Type {
        #[easy_xml(rename = "TTT1")]
        T1,
        #[easy_xml(rename = "TTT2")]
        T2,
        #[easy_xml(rename = "TTT3")]
        T3,
    }

    #[derive(PartialEq, Debug, XmlDeserialize, XmlSerialize)]
    enum Obj {
        #[easy_xml(rename = "TextObject")]
        Text,
        #[easy_xml(rename = "ImgObject")]
        Img,
        #[easy_xml(rename = "VideoObject")]
        Video,
    }
    #[derive(PartialEq, Debug, XmlDeserialize, XmlSerialize)]
    struct Node {
        #[easy_xml(rename = "Type", to_text)]
        ty: Type,
        #[easy_xml(rename = "TextObject|ImgObject|VideoObject")]
        objs: Vec<Obj>,
    }

    let node = Node {
        ty: Type::T2,
        objs: vec![Obj::Text, Obj::Img, Obj::Video],
    };
    let xml = se::to_string(&node).unwrap();

    assert_eq!(
        xml.as_str(),
        r#"<?xml version="1.0" encoding="UTF-8"?><Node><Type>TTT2</Type><TextObject /><ImgObject /><VideoObject /></Node>"#
    );

    assert_eq!(node, de::from_str::<Node>(xml.as_str()).unwrap());
}
