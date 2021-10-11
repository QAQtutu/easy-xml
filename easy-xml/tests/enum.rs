#[macro_use]
extern crate easy_xml_derive;

#[test]
fn test() {
    {
        #[derive(Debug, XmlDeserialize)]
        struct Content {
            #[easy_xml(rename = "Unnamed|Named|Unit", enum)]
            enums: Vec<EnumTest>,
        }
        #[derive(Debug, XmlDeserialize)]
        struct Flatten {
            #[easy_xml(attribute)]
            attr: String,
            #[easy_xml(text)]
            text: String,
        }
        #[derive(Debug, XmlDeserialize)]
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

        for e in content.enums {
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
    }

    {
        #[derive(Debug, XmlDeserialize)]
        enum Test {
            #[easy_xml(prefix = "ofd")]
            T1,
            #[easy_xml(prefix = "ofd")]
            T2,
            #[easy_xml(prefix = "ofd")]
            T3,
        }
    }
}
