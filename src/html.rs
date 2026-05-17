use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::dom::*;

/// HTML文字列を先頭から1文字ずつ読み進めるパーサ。
struct Parser {
    /// パース対象のHTML文字列全体。
    input: String,
    /// 現在の読み取り位置（バイトオフセット）。
    pos: usize,
}

impl Parser {
    /// `pos` の文字を返す。`pos` は進めない。
    fn next_char(&self) -> char {
        if !self.eof()
            && let Some(c) = self.input[self.pos..].chars().next()
        {
            c
        } else {
            panic!("eof!")
        }
    }

    /// `pos` の文字を返し、`pos` を1文字分進める。
    fn consume_char(&mut self) -> char {
        if !self.eof()
            && let Some(c) = self.input[self.pos..].chars().next()
        {
            self.pos += c.len_utf8();
            c
        } else {
            panic!("eof!")
        }
    }

    /// ファイルの終端
    fn eof(&self) -> bool {
        self.pos == self.input.len()
    }

    /// `test` が真を返す間、文字を消費して返す。
    fn consume_while<F: Fn(char) -> bool>(&mut self, test: F) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// 空白の読み飛ばし
    fn skip_whitespace(&mut self) {
        let _ = self.consume_while(char::is_whitespace);
    }

    /// タグ名・属性名を読み取る
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric() || c == '-')
    }

    /// ノード作成
    fn parse_node(&mut self) -> Node {
        if self.next_char() == '<' {
            self.parse_element()
        } else {
            Node::create_text(self.consume_while(|c| c != '<'))
        }
    }

    /// 要素ノード `<tag attr="val">...</tag>` をパースする。
    fn parse_element(&mut self) -> Node {
        // '<'を消費
        let _ = self.consume_char();
        // タグ取得
        let name = self.parse_tag_name();
        // 属性取得
        let attrs = self.parse_attrs();
        // '>'を消費
        let _ = self.consume_char();
        // 子ノード
        let children = self.parse_nodes();
        // 閉じタグ: </タグ名> を消費
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        self.consume_while(|c| c != '>'); // タグ名を読み飛ばす
        assert_eq!(self.consume_char(), '>');

        Node::create_element(name, attrs, children)
    }

    /// 1つの属性 `key="value"` をパースして `(key, value)` を返す。
    fn parse_attr(&mut self) -> (String, String) {
        // キーを読む
        let key = self.parse_tag_name();
        // '=' を消費
        assert_eq!(self.consume_char(), '=');
        // '"' を消費
        assert_eq!(self.consume_char(), '"');
        // '"' まで読む
        let value = self.consume_while(|c| c != '"');
        // '"' を消費
        assert_eq!(self.consume_char(), '"');
        // タプルで返す
        (key, value)
    }

    /// `>` が来るまで属性を繰り返し読み、`HashMap` で返す。
    fn parse_attrs(&mut self) -> HashMap<String, String> {
        let mut attrs = HashMap::new();
        while self.next_char() != '>' {
            self.skip_whitespace();
            let (key, value) = self.parse_attr();
            attrs.insert(key, value);
        }
        attrs
    }

    /// `</` が来るまで子ノードを繰り返し読む。
    fn parse_nodes(&mut self) -> Vec<Rc<RefCell<Node>>> {
        let mut nodes = Vec::new();
        while !self.eof() && !self.input[self.pos..].starts_with("</") {
            self.skip_whitespace();
            if self.eof() || self.input[self.pos..].starts_with("</") {
                break;
            }
            nodes.push(Rc::new(RefCell::new(self.parse_node())));
        }
        nodes
    }
}

pub fn parse(source: String) -> Node {
    let mut parser = Parser {
        input: source,
        pos: 0,
    };
    parser.parse_node()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::NodeType;

    #[test]
    fn test_parse_text() {
        let node = parse("Hello".to_string());
        assert!(matches!(node.node_type, NodeType::Text(ref s) if s == "Hello"));
    }

    #[test]
    fn test_parse_element() {
        let node = parse("<p>Hello</p>".to_string());
        assert!(matches!(
            node.node_type,
            NodeType::Element(ref e) if e.tag_name == "p"
        ));
        assert_eq!(node.children.len(), 1);
    }

    #[test]
    fn test_parse_nested() {
        let node = parse("<html><body><p>Hello</p></body></html>".to_string());
        assert!(matches!(
            node.node_type,
            NodeType::Element(ref e) if e.tag_name == "html"
        ));
        assert_eq!(node.children.len(), 1);
    }

    #[test]
    fn test_parse_attrs() {
        let node = parse("<p class=\"foo\">text</p>".to_string());
        if let NodeType::Element(ref e) = node.node_type {
            assert_eq!(e.attrs.get("class").unwrap(), "foo");
        } else {
            panic!("Element expected");
        }
    }
}
