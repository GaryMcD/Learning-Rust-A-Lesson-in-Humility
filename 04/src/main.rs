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
	
	//println!("Your Input: {}", user_input);

    let message = Message {
    	raw_input : user_input,
    };

    //println!("Number of Blocks in Message: {}", message.number_of_blocks());

    let blocks = message.into_message_blocks();

	//println!("Number of Blocks Returned: {}",blocks.len());

	for block in blocks {
		//println!("{}", block.bits.to_string());
		let words = block.into_words();
		for word in words {
			//println!("{}", word.bits.to_string());
		}
	}

	for val in COMPUTATION_CONSTANTS {
		println!("{}", word::Word::from_u32(val).bits.to_string());
	}
}