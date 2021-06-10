use std::collections::HashMap;

pub struct Node {
  children: Vec<Node>,
  node_type: NodeType,
}
// 要么是文本节点 要么是element
// The following syntax is allowed:
// Balanced tags: <p>...</p>
// Attributes with quoted values: id="main"
// Text nodes: <em>world</em>
enum NodeType {
  Text(String),
  Element(ElementData),
}

struct ElementData {
  tag_name: String,
  attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;
// 生成文本节点？
fn text(data: String) -> Node {
  Node {
    children: Vec::new(),
    node_type: NodeType::Text(data),
  }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children,
    node_type: NodeType::Element(ElementData {
      tag_name: name,
      attributes: attrs,
    }),
  }
}
