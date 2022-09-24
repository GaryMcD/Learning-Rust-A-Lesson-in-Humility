use std::io;

fn main() {
    println!("Input a string you would passed through a SHA-256 hashing algorithm.");

    let mut userInput = String::new();

    io::stdin()
        .read_line(&mut userInput)
        .expect("Failed to read user input");

    println!("Your input was: {userInput}");
}
