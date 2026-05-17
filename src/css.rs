/// タグ名・ID・クラスによる単純セレクタ。
#[derive(Debug)]
pub struct SimpleSelector {
    /// タグ名（例: "h1", "p"）。`None` はワイルドカード。
    pub tag_name: Option<String>,
    /// ID（例: "main"）。
    pub id: Option<String>,
    /// クラスリスト（例: ["foo", "bar"]）。
    pub class: Vec<String>,
}

/// CSSセレクタの種別。
#[derive(Debug)]
pub enum Selector {
    /// 単純セレクタ（タグ名・ID・クラスの組み合わせ）。
    Simple(SimpleSelector),
}

/// CSSスタイルシート全体。複数のルールを持つ。
#[derive(Debug)]
pub struct Stylesheet {
    /// ルールのリスト。
    pub rules: Vec<Rule>,
}

/// 1つのCSSルール（セレクタ群 + 宣言群）。
#[derive(Debug)]
pub struct Rule {
    /// このルールのセレクタリスト。
    pub selectors: Vec<Selector>,
    /// このルールの宣言リスト。
    pub declarations: Vec<Declaration>,
}

/// 1つのCSS宣言（プロパティ名と値のペア）。
#[derive(Debug)]
pub struct Declaration {
    /// プロパティ名（例: "color", "font-size"）。
    pub name: String,
    /// プロパティ値。
    pub value: Value,
}

/// CSS値の種別。
#[derive(Debug)]
pub enum Value {
    /// キーワード値（例: "red", "block"）。
    Keyword(String),
    /// 数値と単位（例: 16px）。
    Length(f32, Unit),
    /// 色値。
    Color(Color),
}

/// CSS長さの単位。
#[derive(Debug)]
pub enum Unit {
    /// ピクセル単位。
    Px,
}

/// RGB色値。各チャンネル 0–255。
#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

struct Parser {
    input: String,
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

    /// 識別子（タグ名・プロパティ名・キーワード値）を読み取る。
    fn parse_identifier(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'))
    }

    /// 単純セレクタ（タグ名・ID・クラス）をパースする。
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: vec![],
        };
        while !self.eof() && self.next_char() != '{' {
            self.skip_whitespace();
            if self.next_char() == '{' {
                break;
            }
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_char();
                }
                _ => {
                    selector.tag_name = Some(self.parse_identifier());
                }
            }
        }
        selector
    }

    fn parse_declaration(&mut self) -> Declaration {
        // プロパティ名
        let name = self.parse_identifier();
        self.skip_whitespace();
        // ':'さくじょ
        self.consume_char();
        self.skip_whitespace();
        // 値
        let value = self.parse_value();
        self.skip_whitespace();
        assert_eq!(self.consume_char(), ';');

        Declaration { name, value }
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    fn parse_length(&mut self) -> Value {
        let num = self
            .consume_while(|c| c.is_numeric() || c == '.')
            .parse::<f32>()
            .unwrap();
        let unit = match self.parse_identifier().as_str() {
            "px" => Unit::Px,
            u => panic!("未知の単位: {}", u),
        };
        Value::Length(num, unit)
    }

    fn parse_color(&mut self) -> Value {
        assert_eq!(self.consume_char(), '#');
        let r = self.parse_hex_pair();
        let g = self.parse_hex_pair();
        let b = self.parse_hex_pair();
        Value::Color(Color { r, g, b })
    }

    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.input[self.pos..self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = vec![];
        loop {
            self.skip_whitespace();
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.skip_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                } // 次のセレクタへ
                '{' => {
                    self.consume_char();
                    break;
                } // ブロック開始
                c => panic!("予期しない文字: {}", c),
            }
        }
        selectors
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = vec![];
        loop {
            self.skip_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        let mut p = Parser {
            input: "#ff0080".to_string(),
            pos: 0,
        };
        if let Value::Color(c) = p.parse_color() {
            assert_eq!(c.r, 255);
            assert_eq!(c.g, 0);
            assert_eq!(c.b, 128);
        } else {
            panic!("Color expected");
        }
    }

    #[test]
    fn test_parse_length() {
        let mut p = Parser {
            input: "16px".to_string(),
            pos: 0,
        };
        if let Value::Length(num, Unit::Px) = p.parse_length() {
            assert_eq!(num, 16.0);
        } else {
            panic!("Length expected");
        }
    }

    #[test]
    fn test_parse_rule() {
        let ss = parse("p { color: red; }".to_string());
        assert_eq!(ss.rules.len(), 1);
        let rule = &ss.rules[0];
        assert_eq!(rule.declarations.len(), 1);
        assert_eq!(rule.declarations[0].name, "color");
        assert!(matches!(rule.declarations[0].value, Value::Keyword(ref s) if s == "red"));
    }

    #[test]
    fn test_parse_selector() {
        let ss = parse("h1 { color: red; }".to_string());
        if let Selector::Simple(ref s) = ss.rules[0].selectors[0] {
            assert_eq!(s.tag_name, Some("h1".to_string()));
        } else {
            panic!("Simple selector expected");
        }
    }

    #[test]
    fn test_parse_multiple_rules() {
        let ss = parse("h1 { color: red; } p { font-size: 16px; }".to_string());
        assert_eq!(ss.rules.len(), 2);
    }
}

/// CSS文字列をパースして `Stylesheet` を返す。
pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser {
        input: source,
        pos: 0,
    };
    let mut rules = vec![];
    while !parser.eof() {
        parser.skip_whitespace();
        if parser.eof() {
            break;
        }
        rules.push(parser.parse_rule());
    }
    Stylesheet { rules }
}
