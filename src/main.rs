//TODO: Enter user inmput and count no of words
//
use std::io;
fn main() {
    println!("Enter a sentence of your choice ? ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Fail to read line");

    let input = input.trim();
    let word_count = input.split_whitespace().count();

    println!("Word count : {}", word_count);
}
