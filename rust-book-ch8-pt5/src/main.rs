use std::collections::HashMap;

fn main() {
    // hash map (kinda like py dicts)

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // sets Blue to 10
    scores.insert(String::from("Blue"), 20); // sets Blue to 20

    scores.entry(String::from("Yellow")).or_insert(30); // sets Yellow to 30
    scores.entry(String::from("Yellow")).or_insert(40); // does nothing cause Yellow already exists

    // example of hash with count of each word
    {
        let text = String::from("hello world wonderful world");

        let mut map = HashMap::new();

        for word  in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1
        }

        println!("{:?}", map)
    }
}
