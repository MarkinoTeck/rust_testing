use std::io;

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    arr[4] = 3;
    println!("{}", arr[4]);

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line :c");
    println!("->> testo: {}", input);
}
