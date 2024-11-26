use std::io;
use ainews_rust::parse_number;

fn main() {
    let mut input = String::new();
    println!("Please enter a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match parse_number(&input) {
        Ok(number) => println!("Your number is: {}", number),
        Err(_) => println!("Please enter a valid number"),
    }
}
