# Refactoring So Early?

It has been a week since the last session and in that time I have spent several hours reading over various `Rust` related artciles and guides. As a result I have decided to refactor my code. My goals are

1) Remove `bitvec` entirely.
2) `MessageBlock` will be a wrapper around a `Vec` of `Word`s.
3) `Word` will be a wrapper around a single `u32`.

The most impacted portion of the code will be when I go from a raw `String` input to a series of `MessageBlock`s. I'm going to take advantage of what `Rust` calls `chunk`s, [Reference](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.chunks). This will allow us to take the collection of `byte`s we get from `String` and `chunk` them by four. Four `u8`s is a single `u32`.

*sidebar*: This would not be an acceptable approach if my input was something other than a string. I know that what I will recieve will always be some multiple of 8 bits in length, so I can make assumptions in my implementation that would not be true if the input could be any random length of bits (like if I was reading a file).

***

## Let's Look At The Code

The `Message`, `MessageBlock`, and `Word` are now:

```Rust
#[derive(Debug)]
pub struct Message {
    pub raw_input: Vec<u8>,
}

#[derive(Debug)]
pub struct MessageBlock {
    pub words: Vec<Word>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Word {
   pub value: u32,
}
```

The juice is in the implementation for `Message`. Within `Message` I have a public method that lets me go from the `Vec<u8>` of input, into the `MessageBlock`s which will be used by the SHA-256 algorithm. Let's take a look.

```Rust
impl Message {
    pub fn into_message_blocks(self) -> Vec<MessageBlock> {
        let (mut words, mut leftovers) = self.into_words_with_leftovers();

        // Pad with single 1 bit (which is 128 in u8)
        {
            leftovers.push(128);
            while leftovers.len() < 4 {
                leftovers.push(0);
            }
            words.push( Word { value: u32::from_be_bytes(leftovers.try_into().unwrap()),})
        }

        // Pad With Zeros
        {
            let number_of_padding_words = 
            16 - (
                (   (words.len() as u64) // current length
                    + 2 ) // length with 64 bit length on end
                % 16); // gap between length and next message block length
                
            for _ in 0..number_of_padding_words {
                words.push(Word { value: 0,});
            }
        }

        // Add 64 bit length on end
        {
            let length = self.raw_input.len() as u64 * 8;

            length
                .to_be_bytes()
                .chunks(4)
                .map(|chunk| Word { value: u32::from_be_bytes(chunk.try_into().unwrap()),})
                .for_each(|word| words.push(word));
        }

        words.chunks(16)
            .map(|words_mb| MessageBlock { words: words_mb.to_vec(),})
            .collect()
    }
}
```

`let (mut words, mut leftovers) = self.into_words_with_leftovers();`. This is my first time implementing `Rust`'s `Tuple`, [Reference](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html). What I am doing here is converting the `Vec<u8>` into useable `Word`s. I cannot ensure that the input will exactly match in length to a series of `u32`s, which is why I have the `leftovers`. This will catch any extra `u8`s on the end of the input. The `into_words_with_leftovers()` method looks like this:

```Rust
fn into_words_with_leftovers(&self) -> (Vec<Word>,Vec<u8>) {
    let mut words: Vec<Word> = Vec::new();
    let mut leftovers: Vec<u8> = Vec::new();

    self.raw_input
        .chunks(4)
        .for_each(|chunk| {
            if chunk.len() == 4 {
                words.push( Word { value: u32::from_be_bytes(chunk.try_into().unwrap()),});
            } else {
                leftovers = chunk.to_vec();
            }
        });
    
    (words, leftovers)
}
```

I declare the collection outputs. I then take the raw_input and `chunk` it. I love this built-in function. :heart: :hocho: It takes a collection and cuts into groups of items of whatever length you tell it to and returns a new collection of these smaller groups. I iterate these `chunk`s and convert them into `Word`s. When I reach the last one, if it is not long enough, I make it the leftovers. :tada: Super Easy! :tada: (and far easier to read than what I previously had). :sunny:

Back in the `into_message_blocks` function. I use the `leftovers` previously allocated to add the required `1` bit onto our input. I pad that with zeros and then add any additional padding necesary.

*sidebar*: I mentioned earlier I get to make assumptions and cheat in my implementation of the algorithm. Padding with zeros is part of that, I will always have to pad with *some* amount of zeros. Perhaps I will create an addition to the extracts document explaining this in greater detail. :pencil2:

Once I have all the required padding done. I can add the 64 bit length onto the collection of words. This is the first time I use `Rust`s `map` function. Essentially, this function allows me to "map" or convert a collection into a new collection of a different type. In this case I took a `u64`, turned it into a collection of `byte`s, chunked those into groups of four (which will only return two results), and `map`ped those chunks into `u32`/`Word`s.

Now I can take this compiled list of `Words` and `chunk` it into groups of 16 (the size of a `MessageBlock`). Take these chunks and `map` them into `MessageBlock`s and call it a day!

***

## Thoughts

This did take a bit of changing around references and muteability until it compiled and ran, but ultimately I think I am getting more comfortable with that side of `Rust`. My problem area this session was with implementing words. `Word { value: u32::from_be_bytes(chunk.try_into().unwrap()),}`. I took the `.try_into().unwrap()` from this [referece](https://doc.rust-lang.org/std/primitive.u32.html#method.from_be_bytes). It mentions that when starting from a slice, which is what a chunk is we must use a "fallible conversion API". I think the point the docs are getting at is that when we chunk we take *references* to the data and do not have ownership and thus cannot convert it directly into a `u32`. Perhaps there is additional nuance in the fact that the `&[u8]` being passed doesn't provide confirmation it is the exact length of four `u8`s. I would think there must be some way to handle this. :interrobang:

A discovery to make some other day. Thanks for tagging along.
