fn main() {
    /*
        Ownership and lifetime
    */
    // copy as stored in stack
    let x = 5;
    let y = x; //copy

    let x = 10;
    let y = some_int(x); // this copyes x into input_int
    println!("x: {x}, y: {y}");

    // move as it's saved on heap
    let s1 = String::from("value");
    let s2 = s1; // move

    // to clone it use the function
    let s1 = String::from("value");
    let s2 = s1.clone(); // clone

    let s1 = String::from("value");
    some_string(s1); // this moves the value into input_string
    // s1 no longer exists here, it is dropped.

    //| references to avoid giving ownership
    // we can make the func take a reference to a string to solve this
    // borrowing the value for the function
    // note that this string is NOT mutable
    let s1 = String::from("value");
    some_string_modified(&s1);
    println!("s1 is still {s1}");

    // mutable example
    let mut s1 = String::from("value");
    change_string(&mut s1);
    println!("s1 is changed {s1}");

    // there can only be one mutable value
    let mut s = String::from("value");
    let r1 = &mut s;
    // let r2 = &mut s1; -> this gives an error

    // but it can be done with unmutable references,
    // as there is no way to corupt data in case of numtiple
    // code changing the value at once
    let s = String::from("value");
    let r1 = &s;
    let r2 = & s1;
}

fn some_int(input_int: i32) -> i32 {
    input_int-1
}

fn some_string(input_string: String) {
    println!("{}", input_string)
}

fn some_string_modified(input_string: &String) {
    println!("{}", input_string)
}

fn change_string(input_string: &mut String) {
    input_string.push_str(" new text");
}