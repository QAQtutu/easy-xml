#[macro_use]
extern crate easy_xml_derive;
use easy_xml::{de, se};
#[allow(unused_variables)]
#[test]
fn test() {
    // struct
    {
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        #[easy_xml(root)]
        struct Person {
            #[easy_xml(flatten)]
            base_info: BaseInfo,
        }

        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        struct BaseInfo {
            #[easy_xml(attribute)]
            age: usize,
            #[easy_xml(rename = "Lang")]
            langs: Vec<String>,
        }

        let person: Person = de::from_str(
            r#"
          <Person age="18">
            <Lang>Chinese</Lang>
            <Lang>English</Lang>
          </Person>
          "#,
        )
        .unwrap();
        assert_eq!(person.base_info.age, 18);
        assert_eq!(person.base_info.langs.len(), 2);

        let xml = se::to_string(&person).unwrap();

        assert_eq!(
            xml.as_str(),
            r#"<?xml version="1.0" encoding="UTF-8"?><Person age="18"><Lang>Chinese</Lang><Lang>English</Lang></Person>"#
        );
    }
    {
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        enum Object {
            Person {
                #[easy_xml(flatten)]
                base_info: BaseInfo,
            },
        }

        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        struct BaseInfo {
            #[easy_xml(attribute)]
            age: usize,
            #[easy_xml(rename = "Lang")]
            langs: Vec<String>,
        }

        let obj: Object = de::from_str(
            r#"
        <Person age="18">
          <Lang>Chinese</Lang>
          <Lang>English</Lang>
        </Person>
        "#,
        )
        .unwrap();
        match &obj {
            Object::Person { base_info } => {
                assert_eq!(base_info.age, 18);
                assert_eq!(base_info.langs.len(), 2);
            }
        }

        let xml = se::to_string(&obj).unwrap();

        assert_eq!(
            xml.as_str(),
            r#"<?xml version="1.0" encoding="UTF-8"?><Person age="18"><Lang>Chinese</Lang><Lang>English</Lang></Person>"#
        );
    }
}
