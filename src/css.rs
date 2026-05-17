/// タグ名・ID・クラスによる単純セレクタ。
struct SimpleSelector {
    /// タグ名（例: "h1", "p"）。`None` はワイルドカード。
    tag_name: Option<String>,
    /// ID（例: "main"）。
    id: Option<String>,
    /// クラスリスト（例: ["foo", "bar"]）。
    class: Vec<String>,
}

/// CSSセレクタの種別。
enum Selector {
    /// 単純セレクタ（タグ名・ID・クラスの組み合わせ）。
    Simple(SimpleSelector),
}

/// CSSスタイルシート全体。複数のルールを持つ。
struct Stylesheet {
    /// ルールのリスト。
    rules: Vec<Rule>,
}

/// 1つのCSSルール（セレクタ群 + 宣言群）。
struct Rule {
    /// このルールのセレクタリスト。
    selectors: Vec<Selector>,
    /// このルールの宣言リスト。
    declarations: Vec<Declaration>,
}

/// 1つのCSS宣言（プロパティ名と値のペア）。
struct Declaration {
    /// プロパティ名（例: "color", "font-size"）。
    name: String,
    /// プロパティ値。
    value: Value,
}

/// CSS値の種別。
enum Value {
    /// キーワード値（例: "red", "block"）。
    Keyword(String),
    /// 数値と単位（例: 16px）。
    Length(f32, Unit),
    /// 色値。
    Color(Color),
}

/// CSS長さの単位。
enum Unit {
    /// ピクセル単位。
    Px,
}

/// RGB色値。各チャンネル 0–255。
struct Color {
    r: u8,
    g: u8,
    b: u8,
}
