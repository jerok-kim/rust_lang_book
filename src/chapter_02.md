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

Next, the line `.read_line(&mut guess)` calls the `read_line` method on the standard input handle to get input from the user. We're also passing `&mut guess` as the argument to `read_line` to tell it what string to store the user input in. The full job of `read_line` is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string's content.

The `&` indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust's major advantages is how safe and easy it is to use references. You don't need to know a lot of those details to finish this program. For now, all we need to know is that, like variables, references are immutable by default. Hence, we need to write `&mut guess` rather than `&guess` to make it mutable.

### Handling Potential Failure with Result

```rust, noplayground
# use std::io;
# 
# fn main() {
#     println!("Guess the number!");
#     println!("Please input your guess.");
# 
#     let mut guess = String::new();
# 
#     io::stdin()
#        .read_line(&mut guess)
         .expect("Failed to read line");
# 
#     println!("You guessed: {guess}");
# }
```

`read_line` puts whatever the user enters into the string we pass to it, but it also returns a `Result` value. `Result` is an enumeration, often called an `enum`, which is a type that can be in one of multiple possible states. We call each possible state a variant.

The purpose of these `Result` type is to encode error-handling information.

`Result`'s variants are `Ok` and `Err`. The `Ok` variant indicates the operation was successful, and inside `Ok` is the successfully generated value. The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.

Values of the `Result` type, like values of any type, have methods defined on them. An instance of `Result` has an `expect` method that you can call. If this instance of `Result` is an `Err` value, `expect` will cause the program to crash and display the message that you passed as an argument to `expect`. If the `read_line` method returns an `Err`, it would likely be the result of an error coming from the underlying operating system. If this instance of `Result` is an `Ok` value, `expect` will take the return value that `Ok` is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user's input.

If you don't call `expect`, the program will compile, but you'll get a warning:

```text
$ cargo build
warning: unused `Result` that must be used
  --> src\main.rs:9:5
   |
9  | /     io::stdin()
10 | |         .read_line(&mut guess);
   | |_______________________________^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
```

Rust warns that you haven't used the `Result` value returned from `read_line`, indicating that the program hasn't handled a possible error.

The right way to suppress the warning is to actually write error-handling code, but in our case we just want to crash this program when a problem occurs, so we can use `expect`.

### Printing Values with println! Placeholders

```rust, noplayground
# use std::io;
# 
# fn main() {
#     println!("Guess the number!");
#     println!("Please input your guess.");
# 
#     let mut guess = String::new();
# 
#     io::stdin()
#        .read_line(&mut guess)
#        .expect("Failed to read line");
# 
      println!("You guessed: {guess}");
# }
```

This line prints the string that now contains the user's input. The `{}` set of curly brackets is a placeholder: think of `{}` as little crab pincers that hold a value in place. When printing the value of a variable, the variable name can go inside the curly brackets. When printing the result of evaluating an expression, place empty curly brackets in the format string, then follow the format string with a comma-separated list of expressions to print in each empty curly bracket placeholder in the same order. Printing a variable and the result of an expression in one call to `println!` would look like this:

```rust, noplayground
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

This code would print x = 5 and y + 2 = 12.

### Testing the First Part

Let's test the first part of the guessing game. Run it using `cargo run`:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (C:\Users\jerok\projects\study_blog\rust_lang_book\code\ch02\guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s                                                               
     Running `target\debug\guessing_game.exe`
Guess the number!
Please input your guess.
3
You guessed: 3
```

## Generating a Secret Number

Next, we need to generate a secret number that the user will try to guess. We'll use a random number between 1 and 100. Rust doesn't yet include random number functionality in its standard library. However, the Rust team does provide a `rand` crate at https://crates.io/crates/rand with said functionality.

### Using a Crate to Get More Functionality

Remember that a crate is a collection of Rust source code files. The project we've been building is a binary crate, which is an executable. The `rand` crate is a library crate, which contains code that is intended to be used in other programs and can't be executed on its own.

Cargo's coordination of external crates is where Cargo really shines. Before we can write code that uses `rand`, we need to modify the `Cargo.toml` file to include the `rand` crate as a dependency. Open that file now and add the following line to the bottom, beneath the `[dependencies]` section header that Cargo created for you. Be sure to specify `rand` exactly as we have here, with this version number, or the code examples in this tutorial may not work.

```toml
[dependencies]
rand = "0.8.5"
```

In the `Cargo.toml` file, everything that follows a header is part of that section that continues until another section starts. In `[dependencies]` you tell Cargo which external crates your project depends on and which versions of those crates you require. In this case, we specify the `rand` crate with the semantic version specifier `0.8.5`. Cargo understands Semantic Versioning(sometimes called SemVer), which is a standard for writing version numbers. The specifier `0.8.5` is actually shorthand for `^0.8.5`, which means any version that is at least 0.8.5 but below 0.9.0.

Cargo considers these versions to have public APIs compatible with version 0.8.5, and this specification ensures you'll get the latest patch release that will still compile with the code in this chapter.

```text
$ cargo build
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.8
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (C:\Users\jerok\projects\study_blog\rust_lang_book\code\ch02\guessing_game)               
    Finished dev [unoptimized + debuginfo] target(s) in 2.00s
```

When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io at https://crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

After updating the registry, Cargo checks the `[dependencies]` section and downloads any crates listed that aren't already downloaded. In this case, although we only listed `rand` as a dependency, Cargo also grabbed other crates that `rand` depends on to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

If you immediately run `cargo build` again without making any changes, you won't get any output aside from the `Finished` line. Cargo knows it has already downloaded and compiled the dependencies, and you haven't changed anything about them in your `Cargo.toml` file. Cargo also knows that you haven't changed anything about your code, so it doesn't recompile that either. With nothing to do, it simply exits.

#### Ensuring Reproducible Builds with the Cargo.lock File

Cargo has a mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, say that next week version 0.8.6 of the `rand` crate comes out, and that version contains an important bug fix, but it also contains a regression that will break your code. To handle this, Rust creates the `Cargo.lock` file the first time you run `cargo build`, so we now have this in the `guessing_game` directory.

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the `Cargo.lock` file. When you build your project in the future, Cargo will see that the `Cargo.lock` file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the `Cargo.lock` file. Because the `Cargo.lock` file is important for reproducible builds, it's often checked into source control with the rest of the code in your project.

#### Updating a Crate to Get a New Version

When you do want to update a crate, Cargo provides the command `update`, which will ignore the `Cargo.lock` file and figure out all the latest versions that fit your specifications in `Cargo.toml`. Cargo will then write those versions to the `Cargo.lock` file. Otherwise, by default, Cargo will only look for versions greater than 0.8.5 and less than 0.9.0.

There's a lot more to say about Cargo and its ecosystem, which we'll discuss in Chapter 14, but for now, that's all we need to know. Cargo makes it very easy to reuse libraries, so Rustaceans are able to write smaller projects that are assembled from a number of packages.

### Generating a Random Number

Let's start using `rand` to generate a number to guess. The next step is to update `src/main.rs`

```rust, editable
#use std::io;
use rand::Rng;

#fn main() {
#    println!("Guess the number!");
#
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
#
#    println!("Please input your guess.");
#
#    let mut guess = String::new();
#
#    io::stdin()
#        .read_line(&mut guess)
#        .expect("Failed to read line");
#
#    println!("You guessed: {guess}");
#}
```