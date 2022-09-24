Today I learn [Rust](https://github.com/rust-lang/rust). Well - not everything there is to learn about Rust. Better said would be: "Today I *begin* learning Rust." Where will this journey take me?

Likely to the edge of madness! Or potentially it will take me back to working primarily in C# where the guardrails are nice and padded.

***

### How will I be learning?

[Languange Documentation](https://www.rust-lang.org/learn#:~:text=Grow%20with%20Rust-,Read,-the%20core%20documentation). It is available online, but as they mention on the website, it can be launched locally with the `rustup doc` command line. Neat little trick.

I will be building an application I have built many times in C#. A SHA-256 Hashing Algorithm. A story for another day would be how in the grief of my mother passing I decided the best way to mourn her would be to lock myself away and spend two months learning `Cuda` and creating a custom Bitcoin Miner. *What a great way to spend my time.* :dissapointed:

***

### How will *it* work?

1. Launch From Command Line
2. No GUI - All Terminal
3. Ask For User Input
4. Return `SHA256(Input)`

***

Let us begin. :rocket:

***

## Setting Up Rust Programming Environment

I am working from within [Ubuntu 22.04 Desktop - Jammy Jellfish](https://releases.ubuntu.com/22.04/) at the moment. So I will follow the steps [here](https://doc.rust-lang.org/book/ch01-01-installation.html) for Linux.

```Bash
> curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh # That is it! Rust is installed.
```
Now for personal stuff.

```Bash
> cd /home/gmcdo/Documents/Coding # Navigate to where I will put code
> mkdir RustLearning
> cd RustLearning 
> mkdir '01 - The Beginning'
> cd '01 - The Beginning' 
> touch README.md # Make the file you are reading right now. So meta.
> cargo init # wait... that didn't work.
```

I receieved the error:
```
error: the name `01 - The Beginning` cannot be used as a package name, the name cannot start with a digit
If you need a package name to not match the directory name, consider using --name flag.
If you need a binary with the name "01 - The Beginning", use a valid package name, and set the binary name to be different from the package. This can be done by setting the binary filename to `src/bin/01 - The Beginning.rs` or change the name in Cargo.toml with:

    [[bin]]
    name = "01 - The Beginning"
    path = "src/main.rs"

```

Ah! Okay, so `cargo init` will use the name of the directory it is in and `Rust` doesn't like numerical digits beginning a package name. No problem, I will set the name manually.

```Bash
> cargo init --name "RustLearning"
> cargo run # Let's see what happens with what it generated.
```

***

## And The Learning Begins

Let us break this down.

```
Compiling RustLearning v0.1.0 (/home/gmcdo/Documents/Coding/RustLearning/01 - The Beginning)
```

Self explanatory. Next...

```
warning: crate `RustLearning` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `rust_learning`
```

Hmm. I am not sure I am a big fan of that - but I don't want to be ostracized by the `Rust` community. So I will have to fix that - but I can wait. Next...

```
warning: `RustLearning` (bin "RustLearning") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/RustLearning`
```

Oh, nice. When it compiles the code and is finished it provides a nice little recap of how the compilation process went and then runs it. I imagine that will become helpful as I learn. Next...

```
Hello, world!
```

And the output. Wonderful.

Let us look at the code in `src`

```Bash
> cd src
> ls
# main.rs
> cat main.rs
```
```Rust
fn main() {
    println!("Hello, world!");
}
```

"`fn main()`" 

This seems obvious enough. Similar to a Console Application in `C#` (that is - prior to the new `.Net` code where they don't have a `main()` in `Program.cs`).

"`println!`"

Uh... what is with that `!`? To Google we go! "`! in rust`" returns:

[Reference](https://doc.rust-lang.org/book/appendix-02-operators.html)
| Operator | Example | Explanation |
| -------- | ------- | ----------- |
| `!` | `ident!(...)` | Macro expansion |

A `Macro`? 

[Looking here](https://doc.rust-lang.org/book/ch19-06-macros.html) we are given some additional context. I won't dive too deep into that at the moment - as an earlier portion of the documentation I have already read mentioned skipping the nuance of macros until Chapter 19 -. So I shall as well.

Before we move on, let's fix that project name issue.

```Bash
> nano ../Cargo.toml
```

Within `Nano` I changed `name = "RustLearning"` to `name = "rust_learning"`. Save and Exit.

And to make sure it doesn't produce the warning anymore.

```Bash
> cd ..
> cargo run
```

No warnings! :star2:

***

## First Bit of Coding

And so it begins. Let us figure out user input. I will imitate what is presented [here](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

```Bash
> nano src/main.rs
```

What I entered...

```Rust
use std::io;

fn main() {
    println!("Input a string you would like passed through a SHA-256 hashing algorithm.");

    io::stdin()
        .read_line(&mut userInput)
        .expect("Failed to read user input");

    println!("Your input was:");
    println!(userInput);
}
```

Does it compile as I expect? Genuinely, as I type this I haven't run it yet - so I am quite excited to see if it works.

```Bash
> cargo run
```

Two errors. :sweat: I expected too much of myself. Let us take a look.

```
error: format argument must be a string literal
  --> src/main.rs:11:14
   |
11 |     println!(userInput);
   |              ^^^^^^^^^
   |
help: you might be missing a string literal to format with
   |
11 |     println!("{}", userInput);
   |              +++++
```

I do have to say, the error readout with help text is.. well... helpful! I could get use to this, especially as a self-teacher and learner who tends to bang my head against walls for hours at length.

Seems we need to adjust the way we print line when not providing a literal string but variable.

Next error.

```
error[E0425]: cannot find value `userInput` in this scope
 --> src/main.rs:7:25
  |
7 |         .read_line(&mut userInput)
  |                         ^^^^^^^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
```

Oh. Duh. A variable has to exist if we are going to assign to it. Seems obvious enough in programming.

Fixed Code:

```Rust
use std::io;

fn main() {
    println!("Input a string you would liked passed through a SHA-256 hashing algorithm.");

    let mut userInput = String::new();

    io::stdin()
        .read_line(&mut userInput)
        .expect("Failed to read user input");

    println!("Your input was: {userInput}");
}
```

```
Input a string you would like passed through a SHA-256 hashing algorithm.
TestInput
Your input was: TestInput
```

:star: It worked! :star:

There was though, a warning during compilation.

```
warning: variable `userInput` should have a snake case name
```

I will fix that. And call it done for the day. Thanks for tagging along on day one of my journey!

```Bash
> cd ..
> git commit -a -m "One Day Of Many"
```