#[macro_use]
extern crate easy_xml_derive;
use easy_xml::de;

#[test]
fn test() {
    {
        #[derive(Debug, XmlDeserialize)]
        struct Content {
            #[easy_xml(rename = "Text|Img", enum)]
            content: Vec<Object>,
        }
        #[derive(Debug, XmlDeserialize)]
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

        for item in content.content {
            match item {
                Object::Text { text } => assert_eq!(text.as_str(), "text"),
                Object::Img { src } => assert_eq!(src.as_str(), "logo.png"),
            }
        }
    }
}
