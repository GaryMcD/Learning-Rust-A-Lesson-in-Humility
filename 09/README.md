# Clean Up & More Testing

I want to get rid of the annoying warnings I currently have. Three of the warnings are because I do not have snake case for all parameters and arguments. The other warning is because of my use of the greek character `σ`. These are all easy resolutions, so I am going to move forward now to testing.

Last time I was able to implement tests for all the word functions and operators. It would be a poor assumption on my part that if those work, everything else must work as well - so I need to implement tests for the message processing logic. That is, how I go from input to a `MessageSchedule`.

When I was first learning how to implement SHA-256 I used this [YouTube Video](https://www.youtube.com/watch?v=f9EbD6iY9zI). About thirteen minutes into the video the narrator shows the process beginnning to end for a sample input. I am going to use that sample input to test my functionality.

The input is `abc`.

In his example that is translated into a bit representation of `011000010110001001100011`. Let's see what I translate `abc` into in binary.

```Rust
// main.rs
let input_bytes = user_input.into_bytes();
	for input_byte in input_bytes {
		println!("{:#010b}", input_byte);
}
```

```
Your input was: abc
0b01100001
0b01100010
0b01100011
```

Well, that looks good! 

## Test Pre-Processing and Padding

So what I need to do now is confirm that the process of padding is done correctly. Considering the SHA-256 process here are the tests I would like to include.

1. `1` bit added on to input.
2. Proper number of `0` bits added for padding.
   - When input is less than 448 bits.
   - When input is greater than 448 bits, but less than 512 bits.
   - When input is greater than 512 bits.
3. Length added to end of padding as 64-bit unsigned integer.
4. Length added is accurate.
5. Final number of message blocks is accurate based on input.
   - Expected result of a single message block.
   - Expected result of more than one message block.

For #1 the test looks like so:

```Rust
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
```

Essentially I hand in a mock input, and then grab the first byte after that input to ensure it equals 128 which is `10000000` in binary, and thus shows only a single `1` was added. I follow a similar pattern for the next three tests. These three tests cover all the remaining circumstances I listed above. It checks the number of blocks, as well as their internal values to ensure we have the right number of `0`s for padding, and the final 64-bit length at the end.

```Rust
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
```

And now when I run tests...

```Bash
> cargo test --lib

...
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Message Schedule

The `MessageSchedule` is a collection of 64 items. Checking that each item is the right value may be programmatically fast, but writing out the list of 64 values by hand to use in the test is not. To my advantage, each value is based on a subset of previous values. So if the final value is correct, I can safely assume the values along the way are as well. If I review the video linked above, his final value (after an input of `abc`) is: `00010010101100011110110111101011`.

Easy enough:

```Rust
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
```

## Hash Computation

Much like the message schedule, there is a lot of work done in this stage of the process, but the final resulting value is so heavily based on the values along the way, that if the final value is correct, it must all be correct. Again, I will borrow the final value from the youtube video and put that in my test.

```Rust
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
```

Well! That covers what I wanted to accomplish this session with testing. See you next time!
