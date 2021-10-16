use easy_xml::se;

#[macro_use]
extern crate easy_xml_derive;

#[test]
fn test_for_option() {
    #[derive(PartialEq, Debug, XmlSerialize)]
    struct Node {
        #[easy_xml(rename = "Child")]
        test: String,
        #[easy_xml(skip)]
        tes1: String,
    }

    let node = Node {
        test: "test".to_string(),
        tes1: "test1".to_string(),
    };
    let xml = se::to_string(&node).unwrap();
    println!("{}", xml);
    assert_eq!(
        xml.as_str(),
        r#"<?xml version="1.0" encoding="UTF-8"?><Node><Child>test</Child></Node>"#
    );
}
