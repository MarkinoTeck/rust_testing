fn main() {
    const SECRET_KEY: &str = "832914689321681932675893467819065189";
    println!("secret: {SECRET_KEY}");

    //| datatypes
    // tuple destructing
    let tup = ("Mark", "mak", 621);
    let (name, _, kills) = tup;
    let username = tup.1;
    println!("{username} ({name}) -> {kills}");

    // arrays (fixed length)
    let arr = [10, 11, 12, 13];
    let first = arr[0];
    println!("first element of list: {first}");

    // empty array [0_i32 x8 times]
    let arr = [0; 8];
    println!("{arr:?}");

    //| funcs
    let result = test_func(2, 5);
    println!("x*y = {}", result);

    // flow control
    let cool_bool = true;
    let num = if cool_bool {10} else {38};
    println!("the num is: {num}");

    //| loop types
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        };
    };

    // while
    let mut num = 3;
    while num != 0 {
        num += 1;
    }

    // for
    let a = [10*100+1; 100];
    for element in a.iter() {
        println!("value: {element}")
    }
    for number in 1..1000 {
        println!("cool value is {}", number)
    }
}

fn test_func(x: i32, y: i32) -> i32 {
    println!("yes, a such a nice function.");
    x*y
}
