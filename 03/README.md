# That Spongebob Meme: "Many Hours Later"

Now would be a good time to share the reference I am using for how to compute SHA-256.

[SHA-256 Process](https://csrc.nist.gov/csrc/media/publications/fips/180/4/final/documents/fips180-4-draft-aug2014.pdf)

Section 5 - PREPROCCESSING is what I will need to accomplish for now. Specifically I will work on 5.1.1.

1. Add binary `1` to end of input.
2. Add binary `0`s to the end of input. The final number of bits needs to be exactly a multiple of 512. I will also be adding 64 bits in step 3. So I will add `0`s until I am 64 bits away from a multiple of 512.
3. Add binary representation of input length in bits, as a 64 bit integer to the end of input.

I am going to be doing a lot of bit specific logic and will also probably need some easy/fast way to check that bits are as I expect. When I did this project in `C#` I used specialized collections of booleans. Doing some research into what `Rust` offers it seems our best option is this crate: [bitvec](https://docs.rs/bitvec/latest/bitvec/).

Let's see if I can figure out with their documentation how to convert what I already have into a collection of bits.

First things first. Add the dependency into `Cargo.toml`

```toml
[dependencies]
bitvec = "1"
```

***

## Time Warp :hourglass_flowing_sand:

So - it is tomorrow since I edited the `Cargo.toml` file a few lines ago. Last night I laid in bed and read [Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html). Such a beautiful thing to read before falling asleep :rose:. I kid, I kid - it was not beautiful, but it was helpful. As a result, I am going to attempt to implement a struct to represent what the SHA-256 documentation calls a `Message` and a `Message Block`.

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
2. Pad the ending as per sha-256 rules. This means I need to take at least a portion of the `raw_input` and **add** bits to it. I have no idea how to do this in `Rust`, even more so with a `BitVec`.
3. Break up the `raw_input` into 512-bit `Message Blocks`. Yet another thing I don't know how to do in `Rust`. I am not familiar with their collection types and how to use them.

Time to read documentation! :scroll:

- [Rust Collections](https://doc.rust-lang.org/std/collections/index.html)
- [BitVec Struct](https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html)
- [What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

Several hours have passed since I went off to read the documentation :clock1:, but I am happy to report I have accomplished my goals and it compiles without errors or warnings! Let me walk you through what I have done.

First, I added a `MessageBlock` `struct`.

```Rust
#[derive(Debug)]
struct MessageBlock {
   bits: BitVec<u8,Msb0>,
}
```

At this time, it just wraps the raw bits, but its good to have it in place because in a future session I will need to add some implementations to it. After creating the `MessageBlock`, I worked on a new method in the `Message` implementation: `fn into_message_blocks(self) -> Vec<MessageBlock>`. It allows us to get a collection of `MessageBlock`s as derived from the user's raw input. Want to see how I did it? Are you ready for spaghetti code? :warning:

First thing to note is that the argument for the method is not `&self`, but `self`. If my understanding is right, this means we don't grab a reference to the `Message`, but instead we grab the `Message` itself. Which will allow us to take ownership of it and it's internal values. Is that right?

Then I started to display my fear of the compiler. :fearful: I saved for later the `original_total_length` because I need that value, but I was afraid that `self` would be gone by the time I needed it. I did the same with `original_number_of_blocks_usize`. Sidenote here...I *sort of* understand the reason for `usize` rather than just a plain integer, but I am not entirely sure of the implications in my use case. If this runs on a machine where references are a different size, does that mean my vectors full of bits will be incorrectly allocated? :question: Maybe someday I will come back to read this and have a better understanding and can answer this for myself.

```Rust
let original_total_length = self.bit_length();
let original_number_of_blocks = self.number_of_blocks();
let original_number_of_blocks_usize = usize::try_from(original_number_of_blocks).unwrap();
```

Now that those variables are out of the way, I created a mutable `BitVec` to hold all the raw bits (`raw_bit_vector`). This will get recycled as I cut parts of it off to make the `MessageBlock`s. As a distance cousin to the `BitVec` I created a mutable `Vec` that will be where I drop off the `MessageBlock`s as they are made. This is ultimately what will be returned from the function.

```Rust
let byte_vector = self.raw_input.into_bytes();
let mut raw_bit_vector = BitVec::<_,Msb0>::from_vec(byte_vector);

let mut blocks: Vec<MessageBlock> = Vec::with_capacity(original_number_of_blocks_usize);
```

These methods I am using, like `try_from` or `unwrap` or `from_vec` or `with_capactity` were all found by scrupulously checking the documentation. Lots of clicking around, skimming articles, and analyzing examples from other programmers. If `Rust` wasn't such a well documented ecosystem I am not sure I could have pulled this off in one multi hour session.

Okay, focus Gary.

Those vectors are in place, now I am ready for a loop. The loop will be going through `raw_bit_vector` cutting off chunks of 512 bits until it has parsed everything into `MessageBlock`s. Within the loop I began by declaring some variables I will initialize later. These will be how I accomplish "recycling" the bits I haven't parsed yet, while working with the bits I need in each iteration of the loop. In most iterations of the loop, I can just cut off 512 bits and make a `MessageBlock` out of it. This doesn't hold up on the final `MessageBlock`.

```Rust
for block_index in 0..original_number_of_blocks {

   let remaining_bit_vector;
   let mut new_block_bit_vector;

   let is_final_block = block_index == (original_number_of_blocks - 1);

   if is_final_block {
      // SNIPPED
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
```

For the final one, I need to do the preprocessing/padding business. This means I need to add a single `1` bit. Then fill it with `0` bits until I have 64 bits left. And finally put the `original_total_length` (that I saved out of fear earlier) in binary on the end of the `MessageBlock` being built.

```Rust
if is_final_block {
   new_block_bit_vector = raw_bit_vector.clone();

   let number_of_zeros = 512 - 64 - 1 - &new_block_bit_vector.len();
   let mut padding_zeros = bitvec![0; number_of_zeros];

   let length_padding = original_total_length.view_bits::<Msb0>();

   new_block_bit_vector.reserve(512 - new_block_bit_vector.capacity()); // Ensure we can hold 512 bits.
   new_block_bit_vector.push(true); // Closing 1
   new_block_bit_vector.append(&mut padding_zeros); // Padding Zeros
   new_block_bit_vector.extend(length_padding); // Encoded length
} else...
```

That last bit honestly was the most painful. It took me a while to figure out the ownership, and which types to use from within the `bitvec` crate. It was messy, and I probably sifted through dozens of compiler errors and warnings before ending up with this functionality. But you know what? It works. And I am not trying to be efficient with my code, so I am happy if it just compiles and runs. At some point I will need to circle back and write tests to confirm the bits are actually as I expected them to be. But that is for another day.

That one function encapsulates what was probably 5+ hours of work on my end. It was incredibly satisfying when I got it to work, but was tiring and frustrating along the way. I am going to call it a night on this one. Thanks for tagging along. :fist:
