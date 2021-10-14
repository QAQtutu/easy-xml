use easy_xml::se;

#[macro_use]
extern crate easy_xml_derive;

#[test]
fn test() {
    {
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        #[easy_xml(prefix="easy",namespace = {"easy":"http://www.easy-xml.org/"})]
        struct Node {
            #[easy_xml(text)]
            text: String,
        }
        let node = Node {
            text: "hello easy-xml".to_string(),
        };
        let xml = se::to_string(&node).unwrap();

        assert_eq!(
            xml.as_str(),
            r#"<?xml version="1.0" encoding="UTF-8"?><easy:Node xmlns:easy="http://www.easy-xml.org/">hello easy-xml</easy:Node>"#
        );
    }
}
