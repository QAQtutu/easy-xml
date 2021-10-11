#[macro_use]
extern crate easy_xml_derive;
use easy_xml::{de, se};

#[test]
fn test() {
    {
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        #[easy_xml(root)]
        struct Content {
            #[easy_xml(rename = "Text|Img", enum)]
            content: Vec<Object>,
        }
        #[derive(Debug, XmlDeserialize, XmlSerialize)]
        enum Object {
            Text {
                #[easy_xml(text)]
                text: String,
            },
            Img {
                #[easy_xml(attribute)]
                src: String,
            },
        }

        let content: Content = de::from_str(
            r#"
          <Content>
            <Text>text</Text>
            <Img src="logo.png"/>
          </Content>
      "#,
        )
        .unwrap();

        assert_eq!(content.content.len(), 2);

        for item in &content.content {
            match item {
                Object::Text { text } => assert_eq!(text.as_str(), "text"),
                Object::Img { src } => assert_eq!(src.as_str(), "logo.png"),
            }
        }

        let xml = se::to_string(&content).unwrap();
        assert_eq!(
            xml.as_str(),
            r#"<?xml version="1.0" encoding="UTF-8"?><Content><Text>text</Text><Img src="logo.png" /></Content>"#
        );
    }
}
