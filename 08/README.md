# Test All The Things

My professional backround is primarily centered around quality assurance and testing, so I am excited for this portion of the learning process.

***

Rant Extracted: [Career Path](https://github.com/GaryMcD/Learning-Rust-A-Lesson-in-Humility/blob/main/Extracts.md#career-path)

***

I will be using this documentation as my starting point: [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html). During my initial read something that stuck out to me was the reference to a `lib.rs` file. I had previously skipped portions of the documentation about managing code as it gets larger, which is where I assume I would have learned more in depth about that. So, to address this I am going to do some code organization now and probably refactoring along the way.

## It Is All Pretty :lipstick:

### Tuple Structs

I ended up getting lost down a rabbit hole learning about [Tuple Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types). I had known of them previously, but this time I realized they would allow me to do what I would like in a prettier, less verbose syntax. That is, "wrap" primitives. Here is an example of what I mean. Previously with `Word` I had this:

```Rust
pub struct Word {
    pub value: u32,
}

impl std::ops::Add<Word> for Word {
   type Output = Word;
   fn add(self, rhs: Word) -> Word {
      Word { value: self.value.wrapping_add(rhs.value) }
   }
}
```

It isn't terribly cumbersome or problematic, but I appreciate the new syntax which allows me to create `Word`s by just doing `Word(x)`. As so:

```Rust
pub struct Word(pub u32);

impl std::ops::Add<Word> for Word {
	type Output = Word;
	fn add(self, rhs: Word) -> Word {
		Word(self.0.wrapping_add(rhs.0))
	}
}
```

The `.0` is a bit odd, but for other structs/code using `Word` like an API, I can abstract that away by implementing the functionality they need from the wrapped `u32` on the `Word` struct directly.

I did this Tuple Struct for all of my structs, as they are all just wrappers of a single thing. I look at it as giving a name and explicit functionality to an otherwise unidentified *thing*.

### Better Organized

The file structure is now as so:

```
src/
   |---Constants.rs
   |---lib.rs
   |---main.rs
   |hash/
   |    |---mod.rs
   |message/
   |       |---message_block.rs
   |       |---message_schedule.rs
   |       |---mod.rs
   |       |---raw_input_handler.rs
   |word/
        |---mod.rs
```

This allows for the main.rs to be a simple binary that calls/uses the lib.rs library. The lib.rs file represents the "API" of what can be done with the code. For a "user" of the code it looks as follows:

```
library rust_learning
	mod Constants:
	   const INITIAL_HASH
	   const COMPUTATION_CONSTANTS
	   
	struct Hash:
		field a Word
		field b Word
		field c Word
		field d Word
		field e Word
		field f Word
		field g Word
		field h Word
		fn compute(Hash, MessageSchedule) -> Hash

	struct MessageBlock
	   field Vec<Word>
	   fn into_message_schedule() -> MessageSchedule

	struct MessageSchedule
		field [Word;64]
		
	struct RawInputHandler
		field Vec<u8>
		fn into_message_blocks() -> Vec<MessageBlock>
		
	struct Word
		field u32
		fn σ_minor() -> Word
		fn σ_major() -> Word
		fn sigma_uc_minor() -> Word
		fn sigma_lc_minor() -> Word
		fn choice(Word, Word, Word) -> Word
		fn majority(Word, Word, Word) -> Word
		fn rotr(u32) -> Word
		fn temporary_word_one(Word, Word, Word, Word, Word, Word) -> Word
		fn temporary_word_two(Word, Word, Word)
```

## Actually Testing

Okay, now that everything is in order, I can begin testing. This process is straightforward and easy :relieved: (as far as I can tell from the documents). In `lib.rs` I just need to add a reference to a test module, and then make a `tests.rs` file in the root of `src`.

```Rust
// lib.rs
#[cfg[test]]
mod tests;
```

Within the `tests.rs` file I can write individual functions that act as individual tests. Each function needs to be marked with `#[test]`, and then if I run `cargo test --lib` it will run all the tests I write and confirm they work as expected.

### Word Testing

First set of tests I put together will be for confirming the functionality around `Word`s. I want to ensure the operator overloading is accurate, then the SHA-256 specific functions (right rotate, choice, σ_minor, etc.).

It's a lot of code to showcase here, so I won't show each test, but here is what I had to do for `choice()` as an example.

```Rust
#[test]
fn word_choice() {
	let a = 	Word(0b00001111_00000000_11111111_11111111);
	let b = 	Word(0b00110011_00000000_11111111_11111111);
	let c = 	Word(0b01010101_00000000_11111111_11111111);
	let result = 	Word(0b01010011_00000000_11111111_11111111);
	assert_eq!(Word::choice(a,b,c), result);
}
```

In the case of SHA-256 a lot of the work is bitwise operations. To make "reading" the test easier I utilized the bit formatting for `u32`. This way I can stack the values atop one another and work out the expected result by hand. The `assert_eq!()` is what is used by `cargo` to confirm if the test is successful or not.

***

I have reached the end of the night and I didn't end up testing all the things. But I did make a good amonut of progress getting my code better organized, and I confirmed all the functions related to individual words is accurate, so that is good!

Thanks for reading along on this session.
