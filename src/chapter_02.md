# Programming a Guessing Game

Let's jump into Rust by working through a hands-on project together! This chapter introduces you to a few common Rust concepts by showing you how to use them in a real program. You'll learn about `let`, `match`, methods, associated functions, external crates, and more.

## Setting Up a New Project

```text
$ cargo new guessing_game
     Created binary (application) `guessing_game` package
```

The `run` command comes in handy when you need to rapidly iterate on a project, as we'll do in this game, quickly testing each iteration before moving on to the next one.

## Processing a Guess

ex 2.1 Code that gets a guess from the user and prints it

```rust, editable
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

To obtain user input and then print the result as output, we need to bring the `io` input/output library into scope. The `io` library comes from the standard library, known as `std:`

```rust
use std::io;
# 
# fn main() {
#     println!("Guess the number!");
#     println!("Please input your guess.");
# 
#     let mut guess = String::new();
# 
#     io::stdin()
#         .read_line(&mut guess)
#         .expect("Failed to read line");
# 
#     println!("You guessed: {guess}");
# }
```

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude, and you can see everything in it at https://doc.rust-lang-org/std/prelude/index.html.

If a type you want to use isn't in the prelude, you have to bring that type into scope explicitly with a `use` statement. Using the `std::io` library provides you with a number of useful features, including the ability to accept user input.

the `main` function is the entry point into the program:

```rust
# use std::io;
# 
fn main() {
#     println!("Guess the number!");
#     println!("Please input your guess.");
# 
#     let mut guess = String::new();
# 
#     io::stdin()
#         .read_line(&mut guess)
#         .expect("Failed to read line");
# 
#     println!("You guessed: {guess}");
# }
```

The `fn` syntax declares a new function; the parentheses, `()`, indicate there are no parameters; and the curly bracket, `{`, starts the body of the function. `println!` is a macro that prints a string to the screen:

```rust
# use std::io;
# 
# fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
# 
#     let mut guess = String::new();
# 
#     io::stdin()
#         .read_line(&mut guess)
#         .expect("Failed to read line");
# 
#     println!("You guessed: {guess}");
# }
```

This code is printing a prompt stating what the game is and requesting input from the user.

### Storing Values with Variables

We'll create a variable to store the user input, like this:

```rust
# use std::io;
# 
# fn main() {
#     println!("Guess the number!");
#     println!("Please input your guess.");
# 
    let mut guess = String::new();
# 
#     io::stdin()
#         .read_line(&mut guess)
#         .expect("Failed to read line");
# 
#     println!("You guessed: {guess}");
# }
```