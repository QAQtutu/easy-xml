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

- [**text**](#text)：字段从节点文本内容中获取。
- [**attribute**](#attribute)：字段从节点参数中获取。
- [**rename**](#rename)：重命名xml节点名称，如果没有则节点名与结构体属性或者枚举值一致。可用`|`分隔来匹配多种节点名称，例如`#[easy_xml(rename = "Text|Img")]`。当使用`|`且在序列化时，节点名称将由字段类型实现自己决定。
- [**prefix**](#prefix)：重命名节点名称前缀。
- [**flatten**](#flatten)：将当前节点传递给字段，即将字段属性展平。
- [**root**](#root)：根节点标记。
- [**namespace**](#namespace):命名空间，仅加在根节点有效。
- [**skip**](#skip): 序列化时跳过字段
- [**to_text**](#to_text): 匹配节点类型后转成文本类型，适合跟枚举类型一起使用。
- [**container**](#container): 标记节点为一个只有名称的容器节点，字段从子元素中获取。目前必须和Vec类型一起使用。

| 属性或类型 | text | attribute | rename | prefix |flatten | root | namespace |skip | to_text | container |
| :-----| ----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: | :----: |
| text | - | N | - | - | - | - | - | - | - |N |
| attribute | N | - | Y | Y | - | - | - | - | - |N |
| rename |  - | Y | - | Y | - | Y | Y | - | Y |- |
| prefix |  - | Y | - | - | - | Y | Y | - | Y |- |
| flatten | - | - | - | - | - | - | - | - | - |N |
| root | - | - | Y | Y | - | - | Y | - | - |- |
| namespace | - | - | Y | Y | - | - | Y | - | - |- |
| skip |  - | - | - | - | - | - | - | - | - |- |
| to_text | - | - | Y | Y | - | - | - | - | - |N |
| container | N | N | - | - | N | - | - | - | N |- |


## 支持计划
- html支持
- xpath部分支持
- 英文文档

## 使用限制
- 结构体属性中Vec与Option不能同时出现，且只能出现在第一层级,且不能多层嵌套。如 `Vec<String>` 和`Option<String>`合法，`Option<Vec<String>>` 是不合法的。
- 如果字段是Vec类型则不能使用text参数，因为节点内容只有一个唯一值。
- 参数attribute、text和flatten不能同时使用。

## 示例

依赖：
```
easy-xml = "0.1.3"
easy-xml-derive = "0.1.3"
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

## 参数详细说明

<a id="text"></a>

- **text** 匹配节点下所有文本节点。

```
<Node>
  <Child>123</Child>
  <Child>456</Child>
</Node>

#[derive(XmlDeserialize,XmlSerialize)]
struct Node {
    #[easy_xml(text)]
    text: String,       //123456
}
```

<a id="attribute"></a>

- **attribute** 属性
```
<Node attr="123456"></Node>

#[derive(XmlDeserialize,XmlSerialize)]
struct Node {
    #[easy_xml(attribute)]
    attr: String,       //123456
}
```
<a id="rename"></a>

- **rename**：重命名xml节点名称
```
<Node>
  <Child>123456</Child>
</Node>

#[derive(XmlDeserialize,XmlSerialize)]
struct Node {
    #[easy_xml(rename="Child")]
    child: String,       //123456
}
```

<a id="root"></a>
<a id="namespace"></a>
<a id="prefix"></a>

- **root**：根节点标记。
- **namespace**:命名空间，仅加在根节点有效。
- **prefix**：重命名节点名称前缀。
```
<easy:Node xmlns:easy="http://easy.org/">
  <easy:Child>123456</easy:Child>
</easy:Node>

#[derive(XmlDeserialize,XmlSerialize)]
#[easy_xml(root,namespace = {"easy":"http://easy.org/"},prefix="easy")]
struct Node {
    #[easy_xml(prefix="easy",rename="Child")]
    child: String,       //123456
}
```

<a id="flatten"></a>

- **flatten**：字段展平。
```
<Node>
  <Child>123456</Child>
</Node>

#[derive(XmlDeserialize,XmlSerialize)]
struct Node {
    #[easy_xml(flatten)]
    flatten:Flatten, 

    //等效于
    // #[easy_xml(rename="Child")]
    // child: String,
}
#[derive(XmlDeserialize,XmlSerialize)]
struct Flatten {
    #[easy_xml(rename="Child")]
    child: String,       //123456
}
```

<a id="skip"></a>

- **skip**: 序列化时跳过字段,仅序列化有效
```
<Node></Node>

#[derive(XmlDeserialize,XmlSerialize)]
struct Node {
    #[easy_xml(skip)]
    child: String,       //123456
}
```

<a id="to_text"></a>

- **to_text**: 匹配节点类型后转成文本类型，适合跟枚举类型一起使用。
```
<Node>
  <Type>T1</Type>
  <T2 />
</Node>

#[derive(XmlDeserialize,XmlSerialize)]
struct Node {
    #[easy_xml(rename="Type",to_text)]
    ty: Type,        //T1
    #[easy_xml(rename="T1|T2|T3")]
    ty2: Type,       //T2
}
enum Type{
  T1,
  T2,
  T3,
}
```

<a id="container"></a>

- **container**: 容器节点标记
```
<Node>
  <Content>
    <Child>123456</Child>
  </Content>
</Node>

#[derive(XmlDeserialize,XmlSerialize)]
struct Node {
    #[easy_xml(container,rename="Content")]
    content: Vec<Child>,       
}
#[derive(PartialEq, Debug, XmlSerialize, XmlDeserialize)]
struct Child {
    #[easy_xml(text)]
    val: String,      //123456
}
```

## 问题记录
- 指针类型未测试

