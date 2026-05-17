use crate::css::*;
use crate::dom::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct StyledNode {
    /// DOMノード
    pub node: Rc<RefCell<Node>>,
    /// 適用されたCSSプロパティ
    pub properties: PropertyMap,
    pub children: Vec<StyledNode>,
}

pub type PropertyMap = HashMap<String, Value>;

pub enum Display {
    Inline,
    Block,
    None,
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // タグ名チェック
    if let Some(ref tag) = selector.tag_name
        && &elem.tag_name != tag
    {
        return false;
    }

    // IDチェック
    if let Some(ref id) = selector.id
        && elem.attrs.get("id") != Some(id)
    {
        return false;
    }

    // クラスチェック
    for class in &selector.class {
        let classes = elem.attrs.get("class").map(|s| s.as_str()).unwrap_or("");
        if !classes.split_whitespace().any(|c| c == class.as_str()) {
            return false;
        }
    }
    true
}

pub fn style_tree(node: Rc<RefCell<Node>>, stylesheet: &Stylesheet) -> StyledNode {
    let properties = match &node.borrow().node_type {
        NodeType::Element(elem) => apply_styles(elem, stylesheet),
        _ => HashMap::new(),
    };
    let children = node
        .borrow()
        .children
        .iter()
        .map(|child| style_tree(Rc::clone(child), stylesheet))
        .collect();
    StyledNode {
        node,
        properties,
        children,
    }
}

fn apply_styles(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut properties = HashMap::new();
    for rule in &stylesheet.rules {
        for selector in &rule.selectors {
            let Selector::Simple(s) = selector;
            if matches_simple_selector(elem, s) {
                for decl in &rule.declarations {
                    properties.insert(decl.name.clone(), decl.value.clone());
                }
            }
        }
    }
    properties
}
