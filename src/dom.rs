use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// HTML要素のタグ名と属性を保持する。
#[derive(Debug)]
pub struct ElementData {
    /// タグ名（例: "div", "p"）
    pub tag_name: String,
    /// 属性マップ（例: "class" -> "foo"）
    pub attrs: HashMap<String, String>,
}

/// DOMノードの種別。
#[derive(Debug)]
pub enum NodeType {
    /// 要素ノード（タグ）
    Element(ElementData),
    /// テキストノード
    Text(String),
    /// コメントノード
    Comment(String),
}

/// DOMツリーの1ノード。
#[derive(Debug)]
pub struct Node {
    /// このノードの種別。
    pub node_type: NodeType,
    /// 子ノードのリスト。
    pub children: Vec<Rc<RefCell<Node>>>,
}
impl Node {
    /// テキストノードを生成する。
    pub fn create_text(data: String) -> Self {
        Self {
            node_type: NodeType::Text(data),
            children: Vec::new(),
        }
    }
    /// 要素ノードを生成する。
    pub fn create_element(
        name: String,
        attrs: HashMap<String, String>,
        children: Vec<Rc<RefCell<Node>>>,
    ) -> Self {
        Self {
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attrs,
            }),
            children,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_text() {
        let node = Node::create_text("Hello".to_string());
        assert!(matches!(node.node_type, NodeType::Text(ref s) if s == "Hello"));
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_create_element() {
        let node = Node::create_element("div".to_string(), HashMap::new(), vec![]);
        assert!(matches!(
            node.node_type,
            NodeType::Element(ref e) if e.tag_name == "div"
        ));
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_create_element_with_attrs() {
        let mut attrs = HashMap::new();
        attrs.insert("class".to_string(), "foo".to_string());
        let node = Node::create_element("p".to_string(), attrs, vec![]);
        if let NodeType::Element(ref e) = node.node_type {
            assert_eq!(e.attrs.get("class").unwrap(), "foo");
        } else {
            panic!("Element expected");
        }
    }
}
