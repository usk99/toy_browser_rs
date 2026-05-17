mod dom;
mod html;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1]; // 最初の引数はプログラム名なので何番目？
    let source = std::fs::read_to_string(path).expect("ファイルが読めません");
    let node = html::parse(source);
    println!("{:#?}", node);
}
