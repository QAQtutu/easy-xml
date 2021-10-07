# easy-xml

基于xml-rs，将文本转换成节点树。通过宏实现将节点树转换成自定义结构体或其他类型。

序列化的宏暂未实现。

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

## 使用限制
- 结构体属性中Vec与Option不能同时出现，且只能出现在第一层级,且不能多层嵌套。如Vec\<String\> 和 Option\<String\>合法，Option\<Vec\<String\>\> 是不合法的。
- 如果字段是Vec类型则不能使用text参数，因为节点内容只有一个唯一值。
- 字段不支持带泛型的结构体，需要给某个具体的类型手动实现easy_xml::XmlDeserialize。


## 简易示例

```
  #[derive(Debug, XmlDeserialize)]
  struct Node {
      #[easy_xml(text)]
      text: String,
  }

  let node: Node = de::from_str("<Node>text</Node>").unwrap();
  assert_eq!(node.text.as_str(), "text");
```

## 自定义

```
  // 反序列化
  impl easy_xml::XmlDeserialize for Node {
      fn deserialize(node: &easy_xml::XmlElement) -> Result<Self, de::Error>
      where
          Self: Sized,
      {
          todo!()
      }
  }
```