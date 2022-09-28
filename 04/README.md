# Can I Compute Yet?

In the last session I got through the task of pre-processing the user's input as a SHA-256 `Message`. That was 5.1.1 in this [document](https://csrc.nist.gov/csrc/media/publications/fips/180/4/final/documents/fips180-4-draft-aug2014.pdf). Now we move onto 5.2. Here we are just breaking up the `MessageBlock`s into sixteen 32-bit `word`s as the document calls them. This shouldn't be too difficult as it will follow a similar process to what we did for chunking the `Message` into `MessageBlock`s.

To do this I will create a `Word` `struct` and then a function inside `MessageBlock` that converts it into a `Vec` of `Word`s.

```Rust
#[derive(Debug)]
struct Word {
  bits: BitVec<u8,Msb0>,
}

impl MessageBlock {
   fn into_words(mut self) -> Vec<Word> {

      let mut words: Vec<Word> = Vec::with_capacity(16);

      for word_index in 0..16 {

         let remaining_bits;
         let word_bits;

         if word_index == 15 {
            word_bits = self.bits.clone();
         } else {
            remaining_bits = self.bits.split_off(32);
            word_bits = self.bits.clone();
            self.bits = remaining_bits.clone();
         }

         let new_word = Word {
            bits: word_bits,
         };

         words.push(new_word);
      }

      words
   }
}
```

And it works! :star:

```
Input a string you would like passed through a SHA-256 hashing algorithm.
>Test
[0, 1, 0, ...//Redacted To Save Space//... 0, 0, 0]
```

At this point I want to start seperating out my code into different files to keep it cleaner and easier to manage. In `C#` I am use to doing each class in it's own file. I am not sure if `Rust` follows the same idealogy. So time to read [documentation](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html).

Okay, I got it to compile without any errors after moving code around, but I am a bit confused. Let me show you what I have and then I will explain my confusion.

```
src/
  |---main.rs
  |    pub mod message;
  |    pub mod word;
  |    fn main() { ... }
  |
  |---message.rs
  |    pub mod message {
  |       pub struct Message { ... }
  |       pub struct MessageBlock { ... }
  |    }
  |
  |---word.rs
  |    pub mod word {
  |       pub struct Word { ... }
  |    }
```

Within the `main()` of `main.rs` I create a `Message`. So I use the `use` keyword to bring `Message` into scope. But the compiler required I specify it as `use message::message::Message;`. As I typed that last sentence, I had an idea. :bulb: One moment, let me try something.

Yup. I fixed my issue. :hammer: I was going to ask, why do I have to declare `message` twice in the path. Turns out that by using `pub mod message` in the `main.rs` it tells the compiler that the code for `mod message` is in the `message.rs` file. So I didn't have to go declare `pub mod message` again in `message.rs`.

New structure:

```
src/
  |---main.rs
  |    pub mod message;
  |    pub mod word;
  |    fn main() { ... }
  |
  |---message.rs
  |    pub struct Message { ... }
  |    pub struct MessageBlock { ... }
  |
  |---word.rs
  |    pub struct Word { ... }
```

I really dont like all the lowercase syntax and standards - *it just sits wrong with me* -.

## Hash Constants :hash:

Now I move onto 5.3.3 and a step I skipped earlier. For the SHA-256 algorithm, there are 64 computation constants used. The document mentioned them in 4.2.2, but I forgot about it. There is also an initial `Hash` used to start the algorithm.

A `Hash` consists of 8 32-bit `Words`.

Here is what I came up with...

```Rust
// Added to src/main.rs
pub mod hash;
pub mod CONSTANTS;

// Added to src/word.rs
impl Word {
    pub fn from_u32(input: u32) -> Word {
        let raw_bytes = input.to_be_bytes().to_vec();
        let word_bits = BitVec::<_,Msb0>::from_vec(raw_bytes);

        let word = Word {
            bits: word_bits
        }

        word
    }
}

// In src/hash.rs
use crate::word::Word;

pub struct Hash {
    a: Word,
    b: Word,
    c: Word,
    d: Word,
    e: Word,
    f: Word,
    g: Word,
    h: Word,
}

// In src/CONSTANTS.rs
pub const INITIAL_HASH_VALUES: [u32; 8] =
[
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19
];

pub const COMPUTATION_CONSTANTS: [u32; 64] =
[
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];
```

There was a bit of horsing around with the `const` keyword to get this to work. Originally I had the constant arrays as being filled with `Word` rather than `u32`. This required calling a function to convert from `u32` to `Word`. That isn't allowed for constants (calling a function from a `const`, unless it is a `const` function, and I didn't want to deal with learning what that means, at least not yet). Instead, as you can see, I just made them arrays of the `u32` value.

It is getting a bit late for me, so, signing off for now.
