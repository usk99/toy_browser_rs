use crate::{css::*, style::*};

/// レイアウトツリーの1ノード。
#[derive(Debug)]
pub struct LayoutBox<'a> {
    /// 対応するスタイルノードへの参照。
    pub styled: &'a StyledNode,
    /// このボックスの寸法。
    pub dimensions: Dimensions,
    /// ボックスの種別。
    pub box_type: BoxType,
    /// 子ボックスのリスト。
    pub children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
    /// ブロックのレイアウトを計算する。
    pub fn layout(&mut self, containing: &Rect) {
        self.calculate_block_width(containing);
        self.calculate_block_position(containing);
        self.layout_children();
        self.calculate_block_height();
    }

    /// 幅を親のcontent幅に合わせる。
    fn calculate_block_width(&mut self, containing: &Rect) {
        let margin_left = self.get_length("margin-left");
        let margin_right = self.get_length("margin-right");
        let padding_left = self.get_length("padding-left");
        let padding_right = self.get_length("padding-right");
        self.dimensions.content.width =
            containing.width - margin_left - margin_right - padding_left - padding_right;
        self.dimensions.padding.left = padding_left;
        self.dimensions.padding.right = padding_right;
        self.dimensions.margin.left = margin_left;
        self.dimensions.margin.right = margin_right;
    }

    /// 位置を親のcontent領域の下端に設定する。
    fn calculate_block_position(&mut self, containing: &Rect) {
        let margin_top = self.get_length("margin-top");
        let padding_top = self.get_length("padding-top");
        self.dimensions.content.x =
            containing.x + self.dimensions.margin.left + self.dimensions.padding.left;
        self.dimensions.content.y = containing.y + containing.height + margin_top + padding_top;
    }

    // properties から px 値を取得するヘルパー。なければ 0.0 を返す。
    fn get_length(&self, name: &str) -> f32 {
        match self.styled.properties.get(name) {
            Some(Value::Length(v, Unit::Px)) => *v,
            _ => 0.0,
        }
    }

    /// 子ボックスを順に並べ、自身の高さを積み上げる。
    fn layout_children(&mut self) {
        for child in &mut self.children {
            child.layout(&self.dimensions.content);
            // 子の高さ分だけ自分のcontentを伸ばす
            self.dimensions.content.height += child.dimensions.content.height;
        }
    }

    /// `height` プロパティが指定されていれば content の高さを上書きする。
    fn calculate_block_height(&mut self) {
        if let Some(Value::Length(h, Unit::Px)) = self.styled.properties.get("height") {
            self.dimensions.content.height = *h;
        }
    }
}

/// ボックスのレイアウト種別。
#[derive(Debug)]
pub enum BoxType {
    Block,
    Inline,
    Anonymous, // インライン要素をまとめる匿名ボックス
}

/// ボックスの寸法（content・padding・border・margin）。
#[derive(Debug)]
pub struct Dimensions {
    pub content: Rect,
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

/// 矩形領域（位置とサイズ）。座標原点は左上、Y軸は下向き。
#[derive(Debug, Default, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// 上下左右の余白サイズ（margin・padding・border 共通）。
#[derive(Debug, Default)]
pub struct EdgeSizes {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

/// スタイルツリーからレイアウトツリーを構築する。
pub fn layout_tree(node: &'_ StyledNode) -> LayoutBox<'_> {
    let dimensions = Dimensions {
        content: Rect::default(),
        padding: EdgeSizes::default(),
        border: EdgeSizes::default(),
        margin: EdgeSizes::default(),
    };

    let box_type = match node.display() {
        Display::Inline => BoxType::Inline,
        Display::Block => BoxType::Block,
        Display::None => panic!("display:nonde は未対応"),
    };

    let children = node.children.iter().map(layout_tree).collect();

    LayoutBox {
        styled: node,
        dimensions,
        box_type,
        children,
    }
}
