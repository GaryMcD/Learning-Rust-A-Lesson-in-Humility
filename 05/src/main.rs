use std::io;
use message::Message;
use CONSTANTS::COMPUTATION_CONSTANTS;

pub mod message;
pub mod word;
pub mod hash;
pub mod CONSTANTS;

fn main() {
    println!("Input a string you would like passed through a SHA-256 hashing algorithm.");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

	// Removes the linefeed
	user_input.truncate(user_input.len()-1);
	
	println!("Your input was: {}", user_input);

	let message = Message { raw_input: user_input.into_bytes(),};

	for block in message.into_message_blocks() {
		for word in block.words {
			println!("Value is {}", word.value);
		}
	}
}