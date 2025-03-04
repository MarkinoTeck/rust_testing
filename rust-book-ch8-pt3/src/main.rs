
fn main() {
    // strings! (utf-8 encoded)
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    // mut strings
    let mut s = String::from("foo");
    s.push_str(" bar");
    s.push('!');
    // foo bar!

    // unite strings
    {
        let mut s1 = String::from("Hello, ");
        let mut s2 = String::from("World!");
        let s3: String = s1 + &s2;
        // s1 is now moved, saving a bit of memory
        // but s1 is not usable anymore.
    }
    {
        let mut s1 = String::from("Hello, ");
        let mut s2 = String::from("World!");
        let s3 = format!("{}{}", s1, s2);
        // the format macro doesn't take ownership
    }

    // iterate over string
    {
        let hello = String::from("Hello!");

        //  iterate over bytes
        for b in hello.bytes() {
            println!("{}", b)
        }
        // iterate chars
        for c in hello.chars() {
            println!("{}", c)
        }
    }
}
