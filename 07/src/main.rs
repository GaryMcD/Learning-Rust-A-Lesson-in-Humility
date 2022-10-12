use std::io;
use message::Message;
use constants::INITIAL_HASH;
use hash::Hash;

pub mod message;
pub mod word;
pub mod hash;
pub mod constants;

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

	let mut hash = INITIAL_HASH;

	for block in message.into_message_blocks() {
		//for word in &block.words {
		//	println!("Value is {}", word.value);
		//}

		//for word in block.into_message_schedule().words {
		//	println!("Value is {}", word.value);
		//}

		hash = Hash::compute(hash, block.into_message_schedule());

		println!("a: {}",hash.a.value);
		println!("b: {}",hash.b.value);
		println!("c: {}",hash.c.value);
		println!("d: {}",hash.d.value);
		println!("e: {}",hash.e.value);
		println!("f: {}",hash.f.value);
		println!("g: {}",hash.g.value);
		println!("h: {}",hash.h.value);
	}
}
