// Creating a guessing game in rust
mod variables;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

use crate::variables::{int_types, name};
fn main() {
    println!("Hello to Guesses");

    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("The secret_number is : {}", secret_number);

    loop {
        println!("Input your guess : ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line ");

        let guess: u32 = guess.trim().parse().expect("Please type a valid integer");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too smalll"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed Correctly");
                break;
            }
        }
    }
    name();
    int_types();
}
