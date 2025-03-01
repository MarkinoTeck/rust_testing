use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    let secret_num = rand::rng().random_range(1..100);
    println!("The secret nummber is {secret_num}, don't tell anyone.");

    loop {
        println!("{}", "Please guess a number".green());

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading console.");
        println!("You tried {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You WIN! :p".green());
                break;
            },
        }
    }
}
