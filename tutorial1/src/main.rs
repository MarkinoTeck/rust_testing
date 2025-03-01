fn main() {
    let x: i64 = -8;
    println!("x is: {}", x);

    {
        let x = x + 1000;
        println!("x is: {x}");
    }

    let x = x + 10;
    println!("x is: {}", x);
}
