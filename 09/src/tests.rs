use super::Word;
use super::MessageBlock;
use super::RawInputHandler;
use super::constants::INITIAL_HASH;
use super::Hash;

#[test]
fn word_equality() {
	let lhs = Word(0);
	let rhs = Word(0);
	assert_eq!(lhs.0 == rhs.0, lhs == rhs);
}

#[test]
fn word_inequality(){
	let lhs = Word(0);
	let rhs = Word(1);
	assert_eq!(lhs.0 == rhs.0, lhs == rhs);
}
 
#[test]
fn word_add() {
	let lhs = Word(0);
	let rhs = Word(5);
	assert_eq!((lhs+rhs), Word(5));
}

#[test]
fn word_add_overflow() {
	let lhs = Word(u32::MAX);
	let rhs = Word(5);
	assert_eq!((lhs+rhs), Word(4))
}

#[test]
fn word_add_compound() {
	let a = Word(0b00001111_00000000_11111111_11111111);
	let b = Word(0b00001111_00000000_11111111_11111111);
	let c = Word(0b00001111_00000000_11111111_11111111); 
	let result = Word(0b00101101_00000010_11111111_11111101);
	assert_eq!(a+b+c, result);
}

#[test]
fn word_shift_left() {
	let lhs = Word(1);
	let rhs = Word(2);
	assert_eq!(lhs << 1, rhs);
	let rhs_2 = Word(4);
	assert_eq!(lhs << 2, rhs_2);
}

#[test]
fn word_shift_right() {
	let lhs = Word(4);
	let rhs = Word(2);
	assert_eq!(lhs >> 1, rhs);
	let rhs_2 = Word(1);
	assert_eq!(lhs >> 2, rhs_2);
}

#[test]
fn word_not() {
	let lhs = Word(0b10101010_10101010_10101010_10101010);
	let rhs = Word(0b01010101_01010101_01010101_01010101);
	assert_eq!(!lhs, rhs);
}

#[test]
fn word_xor() {
	let lhs = 		Word(0b10101010_10101010_10101010_10101010);
	let rhs = 		Word(0b11010100_01010101_01010101_01010101);
	let result = 	Word(0b01111110_11111111_11111111_11111111);
	assert_eq!(lhs ^ rhs, result);
}

#[test]
fn word_or() {
	let lhs = 		Word(0b10101010_10101010_10101010_10101010);
	let rhs = 		Word(0b11010100_01010101_01010101_01010101);
	let result = 	Word(0b11111110_11111111_11111111_11111111);
	assert_eq!(lhs | rhs, result);
}

#[test]
fn word_and() {
	let lhs = 		Word(0b10101011_10101010_10101010_11111111);
	let rhs = 		Word(0b11010101_01010101_01010101_11111111);
	let result = 	Word(0b10000001_00000000_00000000_11111111);
	assert_eq!(lhs & rhs, result);
}

#[test]
fn word_choice() {
	let a = 			Word(0b00001111_00000000_11111111_11111111);
	let b = 			Word(0b00110011_00000000_11111111_11111111);
	let c = 			Word(0b01010101_00000000_11111111_11111111);
	let result = 	Word(0b01010011_00000000_11111111_11111111);
	assert_eq!(Word::choice(a,b,c), result);
}

#[test]
fn word_majority() {
	let a = 			Word(0b00001111_00000000_11111111_11111111);
	let b = 			Word(0b00110011_00000000_11111111_11111111);
	let c = 			Word(0b01010101_00000000_11111111_11111111);
	let result = 	Word(0b00010111_00000000_11111111_11111111);
	assert_eq!(Word::majority(a,b,c), result);
}

#[test]
fn word_rotr() {
	let lhs = Word(0b00001111_00000000_11111111_11111111);
	let rhs = Word(0b11110000_11110000_00001111_11111111);
	assert_eq!(lhs.rotr(4), rhs);
}

#[test]
fn word_σ_minor() {
	let lhs = Word(0b00001111_00000000_11111111_11111111);
	let rhs = Word(0b11000000_00000001_11011101_11000000);
	assert_eq!(lhs.sigma_lc_minor(), rhs);
}

#[test]
fn word_σ_major() {
	let lhs = Word(0b00001111_00000000_11111111_11111111);
	let rhs = Word(0b01100000_00000011_10100110_01011111);
	assert_eq!(lhs.sigma_lc_major(), rhs);
}

#[test]
fn word_sigma_uc_minor() {
	let lhs = Word(0b00001111_00000000_11111111_11111111);
	let rhs = Word(0b00111111_11000111_10111011_11000100);
	assert_eq!(lhs.sigma_uc_minor(), rhs);
}

#[test]
fn word_sigma_uc_major() {
	let lhs = Word(0b00001111_00000000_11111111_11111111);
	let rhs = Word(0b10000011_10100010_00011100_01100111);
	assert_eq!(lhs.sigma_uc_major(), rhs);	
}

#[test]
fn word_temporary_word_one() {
	let e = 			Word(0b00001111_00000000_11111111_11111111);
	let f = 			Word(0b00001111_00000000_11111111_11111111);
	let g = 			Word(0b00001111_00000000_11111111_11111111);
	let h = 			Word(0b00001111_00000000_11111111_11111111);
	let k = 			Word(0b00001111_00000000_11111111_11111111);
	let w = 			Word(0b00001111_00000000_11111111_11111111);
	let result = 	Word(0b10111111_10100110_00011100_01100011);
	assert_eq!(Word::temporary_word_one(e,f,g,h,k,w),result);
}

#[test]
fn word_temporary_word_two() {
	let a = 			Word(0b00001111_00000000_11111111_11111111);
	let b = 			Word(0b00001111_00000000_11111111_11111111);
	let c = 			Word(0b00001111_00000000_11111111_11111111);
	let result = 	Word(0b01001110_11001000_10111011_11000011);
	assert_eq!(Word::temporary_word_two(a,b,c),result);
}

#[test]
fn input_single_one_bit_added_to_end() {
	let mock_input = String::from("abc");
	let raw_input_handler = RawInputHandler(mock_input.into_bytes());
	let result_blocks = raw_input_handler.into_message_blocks();
	let first_result_block = &result_blocks[0];
	let first_word = first_result_block.0[0];
	let first_word_as_bytes = first_word.0.to_be_bytes();
	let expected_byte = first_word_as_bytes[3];
	assert_eq!(128, expected_byte);
}

#[test]
fn input_proper_number_of_zeros_when_less_than_448() {
	let mock_input = String::from("abc");
	let raw_input_handler = RawInputHandler(mock_input.into_bytes());
	let result_blocks = raw_input_handler.into_message_blocks();
	assert_eq!(result_blocks.len() as i32, 1);

	let block = &result_blocks[0];

	for index in 1..14 {
		let word = block.0[index];
		assert_eq!(0, word.0);
	}
}

#[test]
fn input_proper_number_of_zeros_when_greater_than_448_but_less_than_512() {
	let mock_input = String::from("abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcda");
	let raw_input_handler = RawInputHandler(mock_input.into_bytes());
	let result_blocks = raw_input_handler.into_message_blocks();
	assert_eq!(result_blocks.len() as i32, 2);

	let block = &result_blocks[0];

	let second_to_last_word = block.0[14];
	let second_to_last_word_bytes = second_to_last_word.0.to_be_bytes();
	for index in 2..4 {
		let byte = second_to_last_word_bytes[index];
		assert_eq!(byte, 0);
	}

	let last_word = block.0[15];
	assert_eq!(last_word.0, 0);

	let second_block = &result_blocks[1];
	for index in 0..14 {
		let word = second_block.0[index];
		assert_eq!(0, word.0);
	}

	let final_word = second_block.0[15];
	assert_eq!(final_word.0, 456);
}

#[test]
fn input_proper_number_of_zeros_when_greater_than_512() {
	let mock_input = String::from("abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd");
	let raw_input_handler = RawInputHandler(mock_input.into_bytes());
	let result_blocks = raw_input_handler.into_message_blocks();
	assert_eq!(result_blocks.len() as i32, 2);

	let block = &result_blocks[1];

	let first_word = block.0[0];
	let first_word_bytes = first_word.0.to_be_bytes();
	assert_eq!(first_word_bytes, "abcd".as_bytes());

	let second_word = block.0[1];
	assert_eq!(second_word.0, 2147483648);

	for index in 2..14 {
		let word = block.0[index];
		assert_eq!(word.0, 0);
	}

	let final_word = block.0[15];
	assert_eq!(final_word.0, 544);
}

#[test]
fn final_value_of_message_schedule_is_accurate() {
	let mock_input = String::from("abc");
	let raw_input_handler = RawInputHandler(mock_input.into_bytes());
	let result_blocks = raw_input_handler.into_message_blocks();
	assert_eq!(result_blocks.len() as i32, 1);

	let block =  &result_blocks[0];
	let message_schedule = block.into_message_schedule();

	let final_word = message_schedule.0[63];
	let expected_final_word = Word(0b00010010101100011110110111101011);
	assert_eq!(final_word, expected_final_word);
}

#[test]
fn final_hash_of_message_block_is_accurate() {
	let mock_input = String::from("abc");
	let raw_input_handler = RawInputHandler(mock_input.into_bytes());
	let result_blocks = raw_input_handler.into_message_blocks();
	assert_eq!(result_blocks.len() as i32, 1);

	let block =  &result_blocks[0];
	let message_schedule = block.into_message_schedule();

	let hash = INITIAL_HASH;

	let hash = Hash::compute(hash, message_schedule);

	let a_expected_word = Word(0b10111010011110000001011010111111);
	let b_expected_word = Word(0b10001111000000011100111111101010);
	let c_expected_word = Word(0b01000001010000010100000011011110);
	let d_expected_word = Word(0b01011101101011100010001000100011);
	let e_expected_word = Word(0b10110000000000110110000110100011);
	let f_expected_word = Word(0b10010110000101110111101010011100);
	let g_expected_word = Word(0b10110100000100001111111101100001);
	let h_expected_word = Word(0b11110010000000000001010110101101);
	assert_eq!(hash.a, a_expected_word);
	assert_eq!(hash.b, b_expected_word);
	assert_eq!(hash.c, c_expected_word);
	assert_eq!(hash.d, d_expected_word);
	assert_eq!(hash.e, e_expected_word);
	assert_eq!(hash.f, f_expected_word);
	assert_eq!(hash.g, g_expected_word);
	assert_eq!(hash.h, h_expected_word);
}
