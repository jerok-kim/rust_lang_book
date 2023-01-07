# Programming a Guessing Game

Let's jump into Rust by working through a hands-on project together! This chapter introduces you to a few common Rust
concepts by showing you how to use them in a real program. You'll learn about `let`, `match`, methods, associated
functions, external crates, and more.

## Setting Up a New Project

```text
$ cargo new guessing_game
     Created binary (application) `guessing_game` package
```

The `run` command comes in handy when you need to rapidly iterate on a project, as we'll do in this game, quickly
testing each iteration before moving on to the next one.

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

To obtain user input and then print the result as output, we need to bring the `io` input/output library into scope.
The `io` library comes from the standard library, known as `std:`

```rust, noplayground
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

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This
set is called the prelude, and you can see everything in it at https://doc.rust-lang-org/std/prelude/index.html.

If a type you want to use isn't in the prelude, you have to bring that type into scope explicitly with a `use`
statement. Using the `std::io` library provides you with a number of useful features, including the ability to accept
user input.

the `main` function is the entry point into the program:

```rust, noplayground
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

The `fn` syntax declares a new function; the parentheses, `()`, indicate there are no parameters; and the curly
bracket, `{`, starts the body of the function. `println!` is a macro that prints a string to the screen:

```rust, noplayground
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

```rust, noplayground
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

We use the `let` statement to create the variable. Here's another example:

```rust, noplayground
let apples = 5;
```

This line creates a new variable named `apples` and binds it to the value `5`. In Rust, variables are immutable by
default, meaning once we give the variable a value, the value won't change. We'll be discussing this concept in detail
in "Variables and Mutability". To make a variable mutable, we add `mut` before the varaible name:

```rust, noplayground
let apples = 5;  // immutable
let mut bananas = 5;  // mutable
```

`let mut guess` will introduce a mutable variable named `guess`. The equal sign (`=`) tells Rust we want to bind
something to the variable now. On the right of the equal sign is the value that `guess` is bound to, which is the result
of calling `String::new`, a function that returns a new instance of a `String`. `String` is a string type provided by
the standard library that is a growable, UTF-8 encoded bit of text.

The `::` syntax in the `::new` line indicates that `new` is an associated function of the `String` type. An associated
function is a function that's implemented on a type, in this case `String`. This `new` function creates a new, empty
string. You'll find a `new` function on many types because it's a common name for a function that makes a new value of
some kind.

In full, the `let mut guess = String::new();` line has created a mutable variable that is currently bound to a new,
empty instance of a `String`.

### Receiving User Input

Recall that we included the input/output functionality from the standard library with `use std::io;` on the first line
of the program. Now we'll call the `stdin` function from the `io` module, which will allow us to handle user input:

```rust, noplayground
# use std::io;
# 
# fn main() {
#     println!("Guess the number!");
#     println!("Please input your guess.");
# 
#     let mut guess = String::new();
# 
    io::stdin()
        .read_line(&mut guess)
#         .expect("Failed to read line");
# 
#     println!("You guessed: {guess}");
# }
```

If we hadn't imported the `io` library with `use std::io;` at the beginning of the program, we could still use the
function by writing this function call as `std::io::stdin`. The `stdin` function returns an instance
of `std::io::Stdin`, which is a type that represents a handle to the standard input for your terminal.

Next, the line `.read_line(&mut guess)` calls the `read_line` method on the standard input handle to get input from the user.