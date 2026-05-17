use std::{cell::RefCell, rc::Rc};

mod css;
mod dom;
mod html;
mod layout;
mod style;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let html_path = args.get(1).map(String::as_str).unwrap_or("test.html");
    let css_path = args.get(2).map(String::as_str).unwrap_or("test.css");
    let html_source = std::fs::read_to_string(html_path).expect("HTMLファイルが読めません");
    let css_source = std::fs::read_to_string(css_path).expect("CSSファイルが読めません");

    let node = html::parse(html_source);
    let stylesheet = css::parse(css_source);

    println!("=== DOM ===");
    println!("{:#?}", node);
    println!("=== Stylesheet ===");
    println!("{:#?}", stylesheet);

    let root = Rc::new(RefCell::new(node));
    let styled = style::style_tree(root, &stylesheet);
    println!("=== StyledTree ===");
    println!("{:#?}", styled);

    let mut layout = layout::layout_tree(&styled);
    let root_rect = layout::Rect {
        x: 0.0,
        y: 0.0,
        width: 800.0,
        height: 0.0,
    };
    layout.layout(&root_rect);
    println!("=== Layout ===");
    println!("{:#?}", layout);
}
