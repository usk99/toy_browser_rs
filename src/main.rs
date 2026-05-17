mod css;
mod dom;
mod html;

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
}
