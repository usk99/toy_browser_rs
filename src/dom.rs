use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// HTML要素のタグ名と属性を保持する。
struct ElementData {
    /// タグ名（例: "div", "p"）
    tag_name: String,
    /// 属性マップ（例: "class" -> "foo"）
    attrs: HashMap<String, String>,
}

/// DOMノードの種別。
enum NodeType {
    /// 要素ノード（タグ）
    Element(ElementData),
    /// テキストノード
    Text(String),
    /// コメントノード
    Comment(String),
}

/// DOMツリーの1ノード。
pub struct Node {
    /// このノードの種別。
    node_type: NodeType,
    /// 子ノードのリスト。
    children: Vec<Rc<RefCell<Node>>>,
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
