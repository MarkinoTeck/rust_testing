use std::{fs::File, io::{self, Read}};

fn main() {
    // panics if there's an error
    let f = File::open("hello.txt").unwrap();

    // returns the error if ther is one
    fn read_username_from_file(name: &str) -> Result<String, io::Error> {
        let mut s = String::new();
        File::open(name)?.read_to_string(&mut s)?;
        Ok(s)
    }
}
