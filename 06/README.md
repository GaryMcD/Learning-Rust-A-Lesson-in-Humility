# Let's Get Down To Business
###### ... to defeat the huns?

So, I can parse the input into `Word`s, and group them into appropiate `MessageBlock`s. Now I must work on creating what is called a `MessageSchedule` from the `MessageBlock`. This is essentially a list of 64 `Word`s that are uniquely generated from the contents of a single `MessageBlock` through a repetative process of bit manipulation.

1) Create a collection with the 16 `Word`s of the `MessageBlock`.
2) Set a value *index* to 16.
3) Gather *index* - 2 (*w2*), *index* - 7 (*w7*), *index* - 15 (*w15*), *index* - 16 (*w16*) from the collection in step #1.
4) Create a `Word` equal to the result of: σ_major(w2) + w7 + σ_minor(w15) + w16.
5) Append the new `Word` to the collection in step #1.
6) Increment *index* by one.
7) Repeat steps #3 through #6 am additional 47 times.

What are `σ_major()` and `σ_minor()`?

These are logical functions defined within the SHA-256 algorithm. They take as an input a single `Word` and return a new `Word` as the result of a series of bitwise operations. I shall implement those now as extensions to `Word`.

## Operator Overloading in `Rust`

I will use this as a reference for for the bitwise operators available in `Rust`. [Operators](https://doc.rust-lang.org/book/appendix-02-operators.html)

First I will implement `rotr` as defined in the SHA-256 algorithm. [Reference: 2.2.2](https://csrc.nist.gov/csrc/media/publications/fips/180/4/final/documents/fips180-4-draft-aug2014.pdf). `ROTR(n): (x)=(x >> n) ∨ (x << w - n)`. Where `n` = amount to rotate, `x` is `Word`, `w` is 32, `>>` is right-shift, `<<` is left-shift, `v` is inclusive-or.


```Rust
// word.rs

impl Word {
	pub fn rotr(&self, rotation_amount: u32) -> Word {
		let lhs = self >> rotation_amount;
		let rhs = self << (32 - rotation_amount);
		lhs | rhs
	}
}
```

I could have written this so that I used the `u32` wrapped inside of `Word`, but instead I took the opportunity to learn Operator Overloading in `Rust`. [Reference](https://doc.rust-lang.org/rust-by-example/trait/ops.html). This allows me to use the operator symbols like `>>` and `<<` directly on the `Word` struct.

```Rust
// word.rs

impl std::ops::BitOr<Word> for Word {
	type Output = Word;
	fn bitor(self, rhs: Word) -> Word {
		Word { value: self.value | rhs.value }
	}
}

impl std::ops::Shr<u32> for Word {
	type Output = Word;
	fn shr(self, rhs: u32) -> Word {
		Word { value: self.value >> rhs }
	}
}

impl std::ops::Shl<u32> for Word {
	type Output = Word;
	fn shl(self, rhs: u32) -> Word {
		Word { value: self.value << rhs }
	}
}
```

Now I will define `σ_minor()` and `σ_major()` which will also require the use of the exclusive-or.

They are defined in 4.1.2 within the previously linked document.

`σ_minor(x) = rotr_7(x) ⊕ rotr_18(x) ⊕ shr_3(x)` and `σ_major(x) = rotr_17(x) ⊕ rotr_19(x) ⊕ shr_10(x)`. Where `x` is the `Word` being worked on, `rotr_#()` is the rotate-right function, `shr_#()` is the shift-right function, and `⊕` is exclusive-or.

```Rust
// word.rs

impl Word {
	pub fn σ_minor(&self) -> Word {
		let rotr_7 = self.rotr(7);
		let rotr_18 = self.rotr(18);
		let shr_3 = self >> 3;
		rotr_7 ^ rotr_18 ^ shr_3
	}
}

impl std::ops::BitXor<Word> for Word {
	type Output = Word;
	fn bitxor(self, rhs: Word) -> Word {
		Word { value: self.value ^ rhs.value }
	}
}
```

I almost forgot, I will need to overload `+` for `Word` so that I can add them together while building the `MessageSchedule`.

```Rust
// word.rs

impl std::ops::Add<Word> for Word {
	type Output = Word;
	fn add(self, rhs: Word) -> Word {
		Word { value: self.value.wrapping_add(rhs.value) }
	}
}
```

I had to lookup and find `wrapping_add`. This allows me to add two `u32`s that would naturally sum *greater than* the max value of `u32` instead they wrap around and start at 0 again. All the addition in SHA-256 requires this.

## Message Schedule Here I Come

I will now create a function on `MessageBlock` that turns it into a `MessageSchedule`, a wrapper for `[Word; 64]`.

```Rust
// message.rs

#[derive(Debug)]
pub struct MessageSchedule {
	pub words: [Word; 64],
}

impl MessageBlock {
	pub fn into_message_schedule(&self) -> MessageSchedule {
		let mut schedule: [Word; 64] = [Word { value: 0 }; 64];
		
		for index in 0..16 {
			schedule[index] = self.words[index];
		}

		for index in 16..64 {
			let w2 = schedule[index-2];
			let w7 = schedule[index-7];
			let w15 = schedule[index-15];
			let w16 = schedule[index-16];

			schedule[index] = w2.σ_major() + w7 + w15.σ_minor() + w16;
		}

		MessageSchedule { words: schedule }
	}
}

// word.rs

...
#[derive(Copy)]
pub struct Word {
...
```

When I run it and print the message schedule it works! 

The compiler doesn't like my use of Greek characters, but I don't really care about that.
