fn main() {
    /*
        string and arr slices
    */

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
    println!("{:?}", slice);

    let mut s = String::from("hello world");
    // `mut` is here for the s.clear(); but it gives an error
    let entire_string = &s[..];
    let is_same = entire_string == "hello world";
    // s.clear(); //! mutable borrow occurs here, error

    // a string slice is always immutable, so you can't edit the original String

    println!("String: l{0} is_same {2} str: {1}", &s, entire_string, is_same);

    let hello = &s[..5];
    let hello = &s[5..];
    let hello = &s[5..7];
}
