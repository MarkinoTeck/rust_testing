enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main() {
    /* some, something or nothing */

    let x = 5;
    let y = Some(5); // can be i32 or none

    let sum = x + y.unwrap_or(0);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Coin.");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
