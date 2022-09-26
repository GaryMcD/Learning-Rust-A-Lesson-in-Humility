use std::io;

fn bit_length_of_string(input_string: &String) -> i32 {

	let mut input_length : i32 = 0;

	for _ in input_string.bytes() {
		input_length += 8;
	}

	input_length
}

fn main() {
    println!("Input a string you would like passed through a SHA-256 hashing algorithm.");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

	let input_length = bit_length_of_string(&user_input);

	println!("Length of input is: {}", input_length);
}
