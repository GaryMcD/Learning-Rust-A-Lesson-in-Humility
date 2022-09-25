```Bash
.../RustLearning> mkdir '02 - Regret'
.../RustLearning> cp -r '01 - The Beginning/*' '02 - Regret'
.../RustLearning> rm '02 - Regret/README.md'
.../RustLearning> touch '02 - Regret/README.md'
```

Today I will need to figure out how to process and prepare the input from the user. From my experience with make sha-256 hash algorithms in the past, and to make my code (at least early on) more readable, I want to take the string input and convert to some sort of bit representation. So, let's do some research into `Rust`'s types.

`String`s in `Rust`, [reference](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings), is not what I expected. Very nuanced, though after reading the linked chapter and remembering when I was first learning about byte/bit representations of different types I can appreciate the intent of `Rust`'s approach. At the surface, when programming we can find ourselves making a lot of assumptions, but at the implementation level the software/hardware may be handling matters in ways that stand opposed to our assumptions, and `Rust` is trying to save me the headache of figuring out my assumptions are wrong through hours of debugging.

At the end of that chapter they provide two examples for iterating over a `String` that allows us to convert it to a `char` or `byte` like type.

```Rust
// char type
for c in "Зд".chars() {
    println!("{}", c);
}

// byte type
for b in "Зд".bytes() {
    println!("{}", b);
}
```
