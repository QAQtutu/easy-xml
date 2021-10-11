# easy-xml

基于xml-rs，将文本转换成节点树。通过宏实现将节点树转换成自定义结构体或其他类型。

## 支持类型

- 枚举
- 结构体
- Vec
- Option
- 布尔值
- 数字
- 指针 （Box Rc Arc Cell RefCell）

## 支持参数

- **text**：字段从节点文本内容中获取
- **attribute**：字段从节点参数中获取
- **rename**：重命名xml节点名称，如果没有则节点名与结构体属性或者枚举值一致
- **prefix**：重命名节点名称前缀
- **flatten**：将当前节点传递给字段，即将字段属性展平
- **enum**：匹配多个节点名称，需搭配rename使用，例如`#[easy_xml(rename = "Text|Img", enum)]`
- **root**：根节点标记（目前为必须，不会自动给未标记的跟节点指定节点名称）


## 支持计划
- namespace参数支持
- xpath 部分支持

## 使用限制
- 结构体属性中Vec与Option不能同时出现，且只能出现在第一层级,且不能多层嵌套。如 `Vec<String>` 和`Option<String>`合法，`Option<Vec<String>>` 是不合法的。
- 如果字段是Vec类型则不能使用text参数，因为节点内容只有一个唯一值。
- 参数attribute、text和flatten不能同时使用。

## 示例

依赖：
```
easy-xml = "0.1.2-beta"
easy-xml-derive = "0.1.2-beta"
```

使用：
```
#[macro_use]
extern crate easy_xml_derive;

#[derive(Debug, XmlDeserialize,XmlSerialize)]
#[easy_xml(root)]
struct Node {
    #[easy_xml(text)]
    text: String,
}

//反序列化
let node: Node = easy_xml::de::from_str("<Node>text</Node>").unwrap();
assert_eq!(node.text.as_str(), "text");

//序列化
let xml = se::to_string(&node).unwrap();
assert_eq!(
    xml.as_str(),
    r#"<?xml version="1.0" encoding="UTF-8"?><Node>text</Node>"#
);
```

## 自定义

```
//序列化
impl easy_xml::XmlSerialize for Node {
    fn serialize(&self, element: &mut easy_xml::XmlElement)
    where
        Self: Sized,
    {
        todo!()
    }
}
//反序列化
impl easy_xml::XmlDeserialize for Node {
    fn deserialize(node: &easy_xml::XmlElement) -> Result<Self, de::Error>
    where
        Self: Sized,
    {
        todo!()
    }
}
```

## 问题记录
- enum参数暂时只能用于枚举，暂未测试其他类型。如果配合其他类型可能出现错误。
- 指针类型未测试