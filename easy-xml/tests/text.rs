#[macro_use]
extern crate easy_xml_derive;
use easy_xml::{de, se};

#[test]
fn test() {
    // struct
    {
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        #[easy_xml(root)]
        struct Node {
            #[easy_xml(text)]
            text: String,
        }
        let node: Node = de::from_str("<Node>text</Node>").unwrap();
        assert_eq!(node.text.as_str(), "text");

        let xml = se::to_string(&node).unwrap();

        assert_eq!(
            xml.as_str(),
            r#"<?xml version="1.0" encoding="UTF-8"?><Node>text</Node>"#
        );
    }
    {
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        #[easy_xml(root)]
        struct Node {
            #[easy_xml(text)]
            text: String,
        }
        let node: Node = de::from_str("<Node><Node1>text</Node1></Node>").unwrap();
        assert_eq!(node.text.as_str(), "text");

        let xml = se::to_string(&node).unwrap();
        assert_eq!(
            xml.as_str(),
            r#"<?xml version="1.0" encoding="UTF-8"?><Node>text</Node>"#
        );
    }

    //enum

    {
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        enum Node {
            Node {
                #[easy_xml(text)]
                text: String,
            },
        }
        let node: Node = de::from_str("<Node>text</Node>").unwrap();
        match &node {
            Node::Node { text } => {
                assert_eq!(text.as_str(), "text");
            }
        }

        let xml = se::to_string(&node).unwrap();

        assert_eq!(
            xml.as_str(),
            r#"<?xml version="1.0" encoding="UTF-8"?><Node>text</Node>"#
        );
    }
}
