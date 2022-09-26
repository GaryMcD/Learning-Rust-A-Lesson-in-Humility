```Bash
.../RustLearning> mkdir '02 - I Can Write a Function'
.../RustLearning> cp -r '01 - The Beginning/*' '02 - I Can Write a Function'
.../RustLearning> rm '02 - I Can Write a Function/README.md'
.../RustLearning> touch '02 - I Can Write a Function/README.md'
```
***

### What is the user saying?

Today I will need to figure out how to process and prepare the input from the user. From my experience with make sha-256 hash algorithms in the past, and to make my code (at least early on) more readable, I want to take the string input and convert to some sort of bit representation. So, let's do some research into `Rust`'s types.

`String` in `Rust`, [reference](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings), is not what I expected. Very nuanced, though after reading the linked chapter and remembering when I was first learning about byte/bit representations of different types I can appreciate the intent of `Rust`'s approach. At the surface, when programming we can find ourselves making a lot of assumptions, but at the implementation level the software/hardware may be handling matters in ways that stand opposed to our assumptions, and `Rust` is trying to save me the headache of figuring out my assumptions are wrong through hours of debugging.

At the end of that chapter they provide two examples for iterating over a `String` that allows us to convert it to a `char` or `bytes` like type.

```Rust
// char type
for c in "Зд".chars() {
    println!("{}", c);
}

// bytes type
for b in "Зд".bytes() {
    println!("{}", b);
}
```

I am going to try and work with the `bytes` type. I am going to implement their second loop into my code and see what I get.

```Rust
// Added at end of main() in main.rs
for byte in user_input.bytes() {
	println!("{byte}");
}
```

Results in...

```
Input a string you would like passed through a SHA-256 hashing alorithm.
> Test
84
101
115
116
10
```

Okay, that "worked". It didn't compile and freak out. The documentation did say that bytes may not be what we expect. In this case it returned five values even though I only typed in four. My initial thoughts are that `Rust` either encodes some bits at the front or end of the `String` value or the fifth `bytes` is actually the encoding for when I pressed enter on my keyboard. Let's look up these values in UTF-8.

Google Search: "`84 UTF-8`". The second result at the time I searched was this reference [Charset.org](https://www.charset.org/utf-8). Comparing the 5 results my code printed with the chart at the website indicates what is printed is the decimal value, and that the fifth `bytes` is the line feed (user pressing enter).

| Dec | Hex | UTF-8 Hex | Char | Unicode description |
| :-: | :-: | :-------: | :--: | :-----------------: |
| 10 | U+000A | 0A | | Control Character: Line Feed (lf) |
| 84 | U+0054 | 54 | T | Latin Capital Letter T |
| 101 | U+0065 | 65 | e | Latin Small Letter E |
| 115 | U+0073 | 73 | s | Latin Small Letter S |
| 116 | U+0074 | 74 | t | Latin Small Letter T |

***

Contextual Lesson Extracted: [Decimal, Binary, Hexadecimal](https://github.com/GaryMcD/Learning-Rust-A-Lesson-in-Humility/blob/main/Extracts.md#decimal-binary-hexadecimal)

***

### Back To Coding

So it looks like `Rust`'s internals end up converting the string to a series of 8-bit integers. I came to this conclusion because what it printed was the decimal number representation of the charactes in the string, rather than the hexadecimal byte representation. But I wan't to see if I can confirm this.

Google Search: "`rust print the type of a variable`" and I found [this](https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable).

I am going to temporarily add this to my code to see what I get printed out.

```
Input a string you would like passed through a SHA-256 hashing alorithm.
> Test
84
u8
101
u8
115
u8
116
u8
10
u8
```

Ah yes. `u8`, probably means unsigned 8-bit integer. Okay. I can work with this. The first step in a SHA-256 algorithm is to determine the bit length of what was passed into it. My plan here is to create a function that takes in a string and returns its length in bits. The code we stole from stack overflow has an example of functions for us to use. So let's see if I can use that to make my own.

```Rust
use std::io;

fn bit_length_of_string(input_string: &String) {

	i32 input_length = 0;

	for _ in input_string.bytes() {
		input_length += 8;
	}

	println!("Length of input in bits is: {}",input_length);
}

fn main() {
    println!("Input a string you would like passed through a SHA-256 hashing algorithm.");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

	bit_length_of_string(&user_input);
}
```

And...

```
 --> src/main.rs:5:10
  |
5 |     i32 input_length = 0;
  |         ^^^^^^^^^^^^ expected one of `:`, `;`, `=`, `@`, or `|` 
```

Let's try...

```Rust

let mut input_length : i32 = 0;

```

Guess what! It works!

```
Input a string you would like passed through a SHA-256 hashing algorithm.
> Test
Length of input in bits is: 40
```

But I want to return the value 40, not print it out in the function. Using this reference helps me get a grasp of the syntax. [Reference](https://doc.rust-lang.org/rust-by-example/fn.html).

```Rust
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

```

And this works as well. Wahoo. I learned the syntax for functions with returns and parameters in `Rust`.
