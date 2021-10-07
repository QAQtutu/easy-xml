#[macro_use]
extern crate easy_xml_derive;
use easy_xml::de;

#[test]
fn test() {
    // struct
    {
        #[derive(Debug, XmlDeserialize)]
        struct Node {
            #[easy_xml(attribute)]
            attr1: String,
            #[easy_xml(attribute)]
            attr2: String,
        }
        let node: Node = de::from_str("<Node attr1=\"value1\" attr2=\"value2\"></Node>").unwrap();
        assert_eq!(node.attr1.as_str(), "value1");
        assert_eq!(node.attr2.as_str(), "value2");
    }
    //enum

    {
        #[derive(Debug, XmlDeserialize)]
        enum Node {
            Node {
                #[easy_xml(attribute)]
                attr1: String,
                #[easy_xml(attribute)]
                attr2: String,
            },
        }
        let node: Node = de::from_str("<Node attr1=\"value1\" attr2=\"value2\"></Node>").unwrap();
        match node {
            Node::Node { attr1, attr2 } => {
                assert_eq!(attr1.as_str(), "value1");
                assert_eq!(attr2.as_str(), "value2");
            }
        }
    }
}
