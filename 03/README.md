# That Spongebob Meme: "Many Hours Later"

Okay, so I can track the length in bits of the input. Now would be a good time to share the reference I am using for how to compute SHA-256.

[SHA-256 Process](https://csrc.nist.gov/csrc/media/publications/fips/180/4/final/documents/fips180-4-draft-aug2014.pdf)

Section 5 - PREPROCCESSING is what I will need to accomplish for now. Specifically I will work on 5.1.1.

1. Add binary `1` to end of input.
2. Add binary `0`s to the end of input.
   - The final number of bits needs to be a multiple of 512. But I will also be adding 64 bits in step 3. So we will add `0`s until we are 64 bits away from a multiple of 512.
3. Add binary representation of input length in bits, as a 64 bit integer to the end of input.

I am going to be doing a lot of bit specific logic and will also probably need some easy/fast way to check that bits are as I expect. When I did this project in `C#` I used specialized collections of booleans. Doing some research into what `Rust` offers it seems our best option is this crate: [bitvec](https://docs.rs/bitvec/latest/bitvec/).

Let's see if I can figure out with their documentation how to convert what I already have into a collection of bits.

First things first. Add the dependency into `Cargo.toml`

```toml
[dependencies]
bitvec = "1"
```
***

# Time Warp :hourglass_flowing_sand:

So - it is tomorrow since I edited the `Cargo.toml` file a few lines ago. Last night I laid in bed and read [Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html). A beautiful read. I am going to attempt to implement a struct to represent what the SHA-256 documentation calls a `Message` and a `Message Block`.

`Message` is essentially the input. `Message Block` is a 512-bit portion of the `Message`.

Let us begin by just moving what we already have into a `Message` struct. I believe this should work.

```Rust
use std::io;

fn main() {
    println!("Input a string you would like passed through a SHA-256 hashing algorithm.");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

    let message = Message {
    	raw_input : user_input,
    };

	println!("Length of input is: {}", message.bit_length());
}


#[derive(Debug)]
struct Message {
	raw_input: String,
}

impl Message {
	fn bit_length(&self) -> u32 {
		let mut input_length : u32 = 0;

		for _ in self.raw_input.bytes() {
			input_length += 8;
		}

		input_length
	}
}
```

I added a `Message` struct with a `raw_input` field marked as `String`. I then gave `Message` an implementation method `bit_length` which returns the length in bits of it's `raw_input`. Constructed the struct within main, and printed out it's length.

Guess what?! It worked! I am getting so good at this. :star_struck:

This next part. I am not so sure about. I need to:

1. Check the length of `raw_input`
2. Pad the ending as per sha-256 rules.
   - This means I need to take at least a portion of the `raw_input` and *_add_* bits to it. I have no idea how to do this in `Rust`, even more so with a `BitVec`.
3. Break up the `raw_input` into 512-bit `Message Blocks`.
   - Yet another thing I don't know how to do in `rust`. I am not familiar with their collection types and how to use them.

Time to read documentation! :scroll:

[Rust Collections](https://doc.rust-lang.org/std/collections/index.html)

Several hours have passed since I went off to read the documentation :clock1:, but I am happy to report I have accomplished my goals and it compiles without errors or warnings! Let me walk you through what I have done.

First, I added a `MessageBlock` `struct`.
```Rust
#[derive(Debug)]
struct MessageBlock {
	bits: BitVec<u8,Msb0>,
}
```
At this time, it just wraps the raw bits, but its good to have it in place because in a future session I will need to add some implementations to it.

After creating the `MessageBlock`, I worked on a new method in the `Message` implementation: `fn into_message_blocks(self) -> Vec<MessageBlock>`.

It allows us to get a collection of `MessageBlock`s as derived from the user's raw input. Want to see how I did it? Are you ready for spaghetti code? :warning:

```Rust
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
```

Before I even begin explaining all of this I do want to say - I am still not sure when I am "borrowing" or have "ownership" or when variables "move", etc. So some of my code was put in place out of fear of the compiler.

1. First thing to note is that the argument for the method is not `&self`, but `self`. If my understanding is right, this means we don't grab a reference to the `Message`, but the `Message` itself. Which will allow us to take ownership of it and its internal values. Is that right?
2. Saved for later the `original_total_length` because I need that value, but I was afraid that `self` would be gone by the time I needed it.
3. Did the same with `original_number_of_blocks_usize`. I *sort of* understand the reason for `usize` rather than just a plain integer, but I am not entirely sure of the implications in my use case. If this runs on a machine where references are a different size, does that mean my vectors full of bits will be incorrectly allocated? :question: Maybe someday I will come back to read this and have a better understanding and can answer this for myself.
4. Create a mutable `BitVect` to hold all the raw bits (`raw_bit_vector`). This will get recycled as I cut parts of it off to make blocks.
5. Create a mutable `Vec` that will be what I populate with blocks as I iterate the `raw_bit_vector`.
6. Initiate a loop. The loop will be going through `raw_bit_vector` cutting off chunks of 512-bits until it has parsed everything into `MessageBlock`s.
7. Within the loop I began by declaring some variables I will initialize later. These will be how I pass around the recycled bits, new bits, old bits, etc.
8. Check the loop to see if we are on the final block.
   - If we are on the final block we need to do all the special SHA-256 stuff. This entails appending a single `1` bit. Then filling with `0` bits until we have 64-bits left. And then encode the `original_total_length` as a u64 into the end of the `BitVec`.
   - Otherwise we just cut off 512-bits, move references around a bit and move along.
9. Whatever we got setup in the if/else section we now use to create a new `MessageBlock` and drop it into the `Vec` that we will be outputting.

That encapsulates what was probably 5+ hours of work on my end. It was incredibly satisfying when I got it to work in the end. I do appreciate the sense of accomplishment when I reach a learning goal in my self-teaching.

I am going to call it a night on this one. Thanks for tagging along.
