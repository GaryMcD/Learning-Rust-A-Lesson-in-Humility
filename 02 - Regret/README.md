```Bash
.../RustLearning> mkdir '02 - Regret'
.../RustLearning> cp -r '01 - The Beginning/*' '02 - Regret'
.../RustLearning> rm '02 - Regret/README.md'
.../RustLearning> touch '02 - Regret/README.md'
```

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

Google Search: "84 UTF-8". The second result at the time I searched was this reference [Charset.org](https://www.charset.org/utf-8). Comparing the 5 results my code printed with the chart at the website indicates what is printed is the decimal value, and that the fifth `bytes` is the line feed (user pressing enter).

| Dec | Hex | UTF-8 Hex | Char | Unicode description |
| :-: | :-: | :-------: | :--: | :-----------------: |
| 10 | U+000A | 0A | | Control Character: Line Feed (lf) |
| 84 | U+0054 | 54 | T | Latin Capital Letter T |
| 101 | U+0065 | 65 | e | Latin Small Letter E |
| 115 | U+0073 | 73 | s | Latin Small Letter S |
| 116 | U+0074 | 74 | t | Latin Small Letter T |

For anyone following along who is not familiar with bytes, bits, hexademical, decimal, etc. and the relationship of them all. Let me try and give a quick lesson. Bear in my mind, I taught this to myself - so I am not a professor of computer science.

Computers communicate with electricty - they translate the electricty into either a `1` or a `0`. They represent `on` and `off` respectively (though this doesn't necesarily mean no electricity and some electricty, usually its above/below some bound of voltage/amperage).
