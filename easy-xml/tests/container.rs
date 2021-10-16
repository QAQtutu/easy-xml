use easy_xml::{de, se};

#[macro_use]
extern crate easy_xml_derive;

#[test]
fn test_for_vec() {
    #[derive(PartialEq, Debug, XmlSerialize, XmlDeserialize)]
    struct Node {
        #[easy_xml(rename = "Container", container)]
        container: Vec<Child>,
    }
    #[derive(PartialEq, Debug, XmlSerialize, XmlDeserialize)]
    struct Child {
        #[easy_xml(text)]
        val: String,
    }

    let node = Node {
        container: vec![
            Child {
                val: "child1".to_string(),
            },
            Child {
                val: "child2".to_string(),
            },
        ],
    };
    let xml = se::to_string(&node).unwrap();
    assert_eq!(
        xml.as_str(),
        r#"<?xml version="1.0" encoding="UTF-8"?><Node><Container><Child>child1</Child><Child>child2</Child></Container></Node>"#
    );
    assert_eq!(node, de::from_str::<Node>(&xml).unwrap());
}
