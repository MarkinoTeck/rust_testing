fn main() {
    let _print_text: String = print_text();
    println!("{0}, {0}, {0}", _print_text);
}

fn print_text() -> String {
    return "Hello!".to_string();
}