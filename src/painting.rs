use crate::css::*;
use crate::layout::*;

/// 描画命令の種別。
#[derive(Debug)]
pub enum DisplayCommand {
    /// 指定した領域を単色で塗りつぶす。
    SolidColor(Color, Rect),
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
