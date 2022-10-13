use std::io;
use rust_learning::RawInputHandler;
use rust_learning::Constants::INITIAL_HASH;
use rust_learning::Hash;

fn main() {	
    println!("Input a string you would like passed through a SHA-256 hashing algorithm.");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

	// Removes the linefeed
	user_input.truncate(user_input.len()-1);

	println!("Your input was: {}", user_input);

	let mut hash = INITIAL_HASH;

	let handler = RawInputHandler(user_input.into_bytes());

	for block in handler.into_message_blocks() {
		//for word in &block.words {
		//	println!("Value is {}", word.value);
		//}

		//for word in block.into_message_schedule().words {
		//	println!("Value is {}", word.value);
		//}

		hash = Hash::compute(hash, block.into_message_schedule());

		println!("a: {}",hash.a.0);
		println!("b: {}",hash.b.0);
		println!("c: {}",hash.c.0);
		println!("d: {}",hash.d.0);
		println!("e: {}",hash.e.0);
		println!("f: {}",hash.f.0);
		println!("g: {}",hash.g.0);
		println!("h: {}",hash.h.0);
	}
}
