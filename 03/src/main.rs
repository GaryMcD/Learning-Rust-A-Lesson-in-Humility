use std::io;
use bitvec::prelude::*;

fn main() {
    println!("Input a string you would like passed through a SHA-256 hashing algorithm.");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

	// Removes the linefeed
	user_input.truncate(user_input.len()-1);
	
	println!("Your Input: {}", user_input);

    let message = Message {
    	raw_input : user_input,
    };

    println!("Number of Blocks in Message: {}", message.number_of_blocks());

    let blocks = message.into_message_blocks();

	println!("Number of Blocks Returned: {}",blocks.len());

	for block in blocks {
		println!("{}", block.bits.to_string());
	}
}


#[derive(Debug)]
struct Message {
	raw_input: String,
}

#[derive(Debug)]
struct MessageBlock {
	bits: BitVec<u8,Msb0>,
}

impl Message {
	fn bit_length(&self) -> u64 {
		let mut input_length : u64 = 0;

		for _ in self.raw_input.bytes() {
			input_length += 8;
		}

		input_length
	}

	fn number_of_blocks(&self) -> u32 {
		// Size with minimum amount of pre-processing.
		let size_with_preprocessing = self.bit_length() + 1 + 64;

		// + 1 so that values under 512 come back as a single block.
		(size_with_preprocessing as u32 / 512) + 1
	}

	fn into_message_blocks(self) -> Vec<MessageBlock> {

		let original_total_length = self.bit_length();
		let original_number_of_blocks = self.number_of_blocks();
		let original_number_of_blocks_usize = usize::try_from(original_number_of_blocks).unwrap();
		
		let byte_vector = self.raw_input.into_bytes();
		let mut raw_bit_vector = BitVec::<_,Msb0>::from_vec(byte_vector);

		let mut blocks: Vec<MessageBlock> = Vec::with_capacity(original_number_of_blocks_usize);

		for block_index in 0..original_number_of_blocks {

			let remaining_bit_vector;
			let mut new_block_bit_vector;

			let is_final_block = block_index == (original_number_of_blocks - 1);
			
			if is_final_block {
				new_block_bit_vector = raw_bit_vector.clone();

				let number_of_zeros = 512 - 64 - 1 - &new_block_bit_vector.len();
				let mut padding_zeros = bitvec![0; number_of_zeros];

				let length_padding = original_total_length.view_bits::<Msb0>();

				new_block_bit_vector.reserve(512 - new_block_bit_vector.capacity()); // Ensure we can hold 512 bits.
				new_block_bit_vector.push(true); // Closing 1
				new_block_bit_vector.append(&mut padding_zeros); // Padding Zeros
				new_block_bit_vector.extend(length_padding); // Encoded length
			} else {
				remaining_bit_vector = raw_bit_vector.split_off(512);
				new_block_bit_vector = raw_bit_vector.clone();
				raw_bit_vector = remaining_bit_vector.clone();
			}

			let new_block = MessageBlock {
				bits: new_block_bit_vector,
			};

			blocks.push(new_block);
		};

		blocks
	}
}
