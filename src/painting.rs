use crate::css::*;
use crate::layout::*;

/// 描画命令の種別。
#[derive(Debug)]
pub enum DisplayCommand {
    /// 指定した領域を単色で塗りつぶす。
    SolidColor(Color, Rect),
    /// テキストを指定位置に描画する。
    Text {
        /// 描画する文字列。
        text: String,
        /// 描画開始X座標（左端）。
        x: f32,
        /// ベースラインのY座標。
        y: f32,
        /// 文字色。
        color: Color,
        /// フォントサイズ（ピクセル）。
        font_size: f32,
    },
}

/// 描画命令のリスト。先頭から順に描画する。
pub type DisplayList = Vec<DisplayCommand>;

/// レイアウトツリーを走査して描画命令リストを生成する。
pub fn build_display_list(layout: &LayoutBox) -> DisplayList {
    let mut list = vec![];
    render_box(&mut list, layout);
    list
}

/// 1つのボックスと子ボックスを再帰的に描画命令に変換する。
fn render_box(list: &mut DisplayList, layout: &LayoutBox) {
    render_background(list, layout);
    render_text(list, layout);
    for child in &layout.children {
        render_box(list, child);
    }
}

/// `background-color` プロパティがあれば `SolidColor` 命令を追加する。
fn render_background(list: &mut DisplayList, layout: &LayoutBox) {
    if let Some(Value::Color(color)) = layout.styled.properties.get("background-color") {
        list.push(DisplayCommand::SolidColor(
            *color,
            layout.dimensions.content,
        ));
    }
}

/// テキストノードがあれば `Text` 命令を追加する。
fn render_text(list: &mut DisplayList, layout: &LayoutBox) {
    if let crate::dom::NodeType::Text(text) = &layout.styled.node.borrow().node_type {
        list.push(DisplayCommand::Text {
            text: text.clone(),
            x: layout.dimensions.content.x + 8.0,
            y: layout.dimensions.content.y + 18.0,
            color: Color::default(),
            font_size: 18.0,
        })
    }
}
