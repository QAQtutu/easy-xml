use easy_xml::{de, se};

#[macro_use]
extern crate easy_xml_derive;

#[test]
fn test_for_option() {
    #[derive(PartialEq, Debug, XmlDeserialize, XmlSerialize)]
    struct Node {
        #[easy_xml(rename = "Child")]
        test: Option<String>,
    }

    let node = Node { test: None };
    let xml = se::to_string(&node).unwrap();
    assert_eq!(
        xml.as_str(),
        r#"<?xml version="1.0" encoding="UTF-8"?><Node />"#
    );
    assert_eq!(node, de::from_str::<Node>(xml.as_str()).unwrap());

    let node = Node {
        test: Some(String::from("test")),
    };
    let xml = se::to_string(&node).unwrap();
    assert_eq!(
        xml.as_str(),
        r#"<?xml version="1.0" encoding="UTF-8"?><Node><Child>test</Child></Node>"#
    );
    assert_eq!(node, de::from_str::<Node>(xml.as_str()).unwrap());
}
