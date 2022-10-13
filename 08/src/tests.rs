use super::Word;

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
	assert_eq!(lhs.σ_minor(), rhs);
}

#[test]
fn word_σ_major() {
	let lhs = Word(0b00001111_00000000_11111111_11111111);
	let rhs = Word(0b01100000_00000011_10100110_01011111);
	assert_eq!(lhs.σ_major(), rhs);
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

