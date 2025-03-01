fn main() {
    test_one();
    test_one();
    // this is an expression
    add_nums(1, 8);

    // this is a statement
    let x = 20;

    // this is an expression
    let num = {
        let x = 3i8;
        x as i32 + 8
    };

    println!("multiply func: {}", multiply(8, 8));
    println!("multiply return func: {}", multiply_with_return(8, 8));
    println!("x: {x}, num: {num}");
}
// this is a statement
fn test_one() {
    println!("Test one has been called")
}
// this is a statement
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}
// this is a statement
fn multiply_with_return(x: i32, y: i32) -> i32 {
    return x * y;
}
// this is a statement
fn add_nums(x: i32, y: i32) {
    println!("sum is: {}", x+y)
}