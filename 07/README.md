# Hash Computation

I am going to get it done now. This is where the fun happens. I'm going to take the `MessageSchedule` and compress into a hash output. The output is either passed into the next block as an input or used for the very final output.

Accomplishing this requires an understanding of a `Hash`. In a previous session I made the struct for it, but I didn't do anything with it. A `Hash` is eight "working" `Word`s that are used in the computation loop. Every iteration of the loop the `Hash` and it's underlying working variables (`Word`s) are re-computed. Their current state is fed into a series of functions and a new state is spit out. I am not a professional at crypotgraphy or hashing algorithms, but I believe this iterative loop is considered a key part of "compression".

As part of the compression/computation loop each iteration requires the creation of two temporary `Word`s that will only be used for the lifecycle of that single iteration. I will begin by creating the functions for those. Much like the last session I will need to setup some other smaller funtions.

- `∑_minor()` and `∑_major()`
- `Choice()`
- `Majority()`

## `∑_minor()` and `∑_major()`

This will be simple as the underlying functions are already in place. The same ones used for `σ_minor()` and `σ_major()`.

```Rust
// word.rs

impl Word {
   pub fn ∑_minor(&self) -> Word {
      let rotr_2 = self.rotr(2);
      let rotr_13 = self.rotr(13);
      let rotr_22 = self.rotr(22);
      rotr_2 ^ rotr_13 ^ rotr_22
   }
   pub fn ∑_major(&self) -> Word {
      let rotr_6 = self.rotr(6);
      let rotr_11 = self.rotr(11);
      let rotr_25 = self.rotr(25);
      rotr_6 ^ rotr_11 ^ rotr_25
   }
}
```

As it turns out, `Rust` does not like the use of ∑ in my code. 

```
error: unknown start of token: \u{2211}
  --> src/word.rs:21:9
   |
21 |     pub fn ∑_minor(&self) -> Word {
   |            ^

```

I guess I will change it.

## `Choice()`

This requires some new operator overloads I have yet to implement, specifically bitwise-and as well as bitwise-complement.

```Rust
// word.rs

impl Word {
   pub fn Choice(x: Word, y: Word, z: Word) -> Word {
      (x & y) ^ ((!x) & z)
   }
}

impl std::ops::BitAnd<Word> for Word {
   type Output = Word;
   fn bitand(self, rhs: Word) -> Word {
      Word { value: self.value & rhs.value }
   }
}

impl std::ops::Not for Word {
   type Output = Word;
   fn not(self) -> Word {
      Word { value: !self.value }
   }
}
```

## `Majority()`

All the required components are in place for this. So I can directly implement this function without needing to add anything else.

```Rust
// word.rs

impl Word {
   pub fn majority(x: Word, y: Word, z: Word) -> Word {
      (x & y) ^ (x & z) ^ (y & z)
   }
}
```

## Temporary Words

I had to modify some of my ownership and references to get this working. I guess I will find out later when I do testing if the results are as I expect.

```Rust
// word.rs

impl Word {
   pub fn temporary_word_one(e: Word, f: Word, g: Word, h: Word, K: Word, W: Word) -> Word {
      h + Word::sigma_uc_major(e) + Word::choice(e,f,g) + K + W
   }
   pub fn temporary_word_two(a: Word, b: Word, c: Word) -> Word {
      Word::sigma_uc_minor(a) + Word::majority(a,b,c)
   }
}
```

## Hash Computation

And now the computation loop.

```Rust
impl Hash {
   pub fn compute(initial_hash: Hash, message_schedule: MessageSchedule) -> Hash {
      let mut working_hash = initial_hash.clone();
		
      for index in 0..64 {
         let t1 = Word::temporary_word_one(
            working_hash.e,
            working_hash.f,
            working_hash.g,
            working_hash.h,
            COMPUTATION_CONSTANTS[index],
            message_schedule.words[index]
         );
         let t2 = Word::temporary_word_two(
            working_hash.a,
            working_hash.b,
            working_hash.c,
         );

         working_hash = Hash {
            a: t1 + t2,
            b: working_hash.a,
            c: working_hash.b,
            d: working_hash.c,
            e: working_hash.d + t1,
            f: working_hash.e,
            g: working_hash.f,
            h: working_hash.g
         };
      }

      Hash {
         a: initial_hash.a + working_hash.a,
         b: initial_hash.b + working_hash.b,
         c: initial_hash.c + working_hash.c,
         d: initial_hash.d + working_hash.d,
         e: initial_hash.e + working_hash.e,
         f: initial_hash.f + working_hash.f,
         g: initial_hash.g + working_hash.g,
         h: initial_hash.h + working_hash.h,
      }
   }
}
```

It is 2am, I got the code to compile and run and spit out results. Next session I will implement testing to confirm the results are accurate.
