use std::collections::HashMap;

fn main() {
    // hash maps
    // store key-value pairs of any type

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    // .insert takes ownerchip
    // println!("{}", blue) -> won't work as value is owned by scores.insert()

    // iterate over it
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
}
