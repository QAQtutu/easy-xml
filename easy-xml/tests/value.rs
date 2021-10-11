#[macro_use]
extern crate easy_xml_derive;
use easy_xml::de;

#[test]
fn test() {
    // struct
    {
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        struct Node {
            #[easy_xml(text)]
            text: String,
        }
        let node: Node = de::from_str("<Node>text</Node>").unwrap();
        assert_eq!(node.text.as_str(), "text");
    }
    {
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        struct Node {
            #[easy_xml(text)]
            text: String,
        }
        let node: Node = de::from_str("<Node><Node1>text</Node1></Node>").unwrap();
        assert_eq!(node.text.as_str(), "text");
    }

    //enum

    {
        #[derive(Debug, XmlDeserialize)]
        enum Node {
            Node {
                #[easy_xml(text)]
                text: String,
            },
        }
        let node: Node = de::from_str("<Node>text</Node>").unwrap();
        match node {
            Node::Node { text } => {
                assert_eq!(text.as_str(), "text");
            }
        }
    }
}
