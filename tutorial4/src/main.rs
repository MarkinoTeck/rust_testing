use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no line!!!!");

    let int_in: i64 = input.trim().parse().unwrap();
    println!("{}", int_in+2)
}
