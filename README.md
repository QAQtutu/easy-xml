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

- **text**：字段从节点文本内容中获取。
- **attribute**：字段从节点参数中获取。
- **rename**：重命名xml节点名称，如果没有则节点名与结构体属性或者枚举值一致。可用`|`分隔来匹配多种节点名称，例如`#[easy_xml(rename = "Text|Img")]`。当使用`|`且在序列化时，节点名称将由字段类型实现自己决定。
- **prefix**：重命名节点名称前缀。
- **flatten**：将当前节点传递给字段，即将字段属性展平。
- **root**：根节点标记。
- **namespace**:命名空间，仅加在根节点顶部有效。
- **skip**: 序列化时跳过字段
- **to_text**: 匹配节点类型后转成文本类型，适合跟枚举类型一起使用。


## 支持计划
- html支持
- xpath部分支持

## 使用限制
- 结构体属性中Vec与Option不能同时出现，且只能出现在第一层级,且不能多层嵌套。如 `Vec<String>` 和`Option<String>`合法，`Option<Vec<String>>` 是不合法的。
- 如果字段是Vec类型则不能使用text参数，因为节点内容只有一个唯一值。
- 参数attribute、text和flatten不能同时使用。

## 示例

依赖：
```
easy-xml = "0.1.2"
easy-xml-derive = "0.1.2"
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
let xml = easy_xml::se::to_string(&node).unwrap();
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
    fn deserialize(node: &easy_xml::XmlElement) -> Result<Self, easy_xml::de::Error>
    where
        Self: Sized,
    {
        todo!()
    }
}
```

## 问题记录
- 指针类型未测试