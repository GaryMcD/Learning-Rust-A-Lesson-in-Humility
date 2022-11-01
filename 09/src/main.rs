use std::io;
use rust_learning::RawInputHandler;
use rust_learning::constants::INITIAL_HASH;
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

	//let input_bytes = user_input.into_bytes();
	//for input_byte in input_bytes {
	//	println!("   {:#010b}", input_byte);
	//}

	let handler = RawInputHandler(user_input.into_bytes());
	
	let mut hash = INITIAL_HASH;

	//for block in handler.into_message_blocks() {

	//	hash = Hash::compute(hash, block.into_message_schedule());

	//	println!("a: {}",hash.a.0);
	//	println!("b: {}",hash.b.0);
	//	println!("c: {}",hash.c.0);
	//	println!("d: {}",hash.d.0);
	//	println!("e: {}",hash.e.0);
	//	println!("f: {}",hash.f.0);
	//	println!("g: {}",hash.g.0);
	//	println!("h: {}",hash.h.0);
	//}
}
