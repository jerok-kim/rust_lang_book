# Getting Started

Let's start our Rust jorney! There's a lot to learn, but every journey starts somewhere. In this chapter, we'll discuss:

- Installing Rust on Linux, macOS, and Windows
- Writing a program that prints `Hello, world!`
- Using `cargo`, Rust's package manager and build system

## Installiation

The first step is to install Rust. We'll download Rust through `rustup`, command line tool for managing Rust versions
and associated tools.

The following steps install the latest stable version of the Rust compiler.
Rust's stability guarantees ensure that all the examples in the book that compile will continue to compile with newer
Rust versions.

### Installing rustup on Linux or macOS

```text
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

### Installing rustup on Windows

[installing rustup](https://www.rust-lang.org/tools/install)<br>
[installing visualstudio](https://visualstudio.microsoft.com/downloads)

### Troubleshooting

```text
$ rustc --version
```

### Updating and Uninstalling

```text
$ rustup update
```

### Local Documentation

The installation of Rust also includes a local copy of the documentation so that you can read it offline.
Run `rustup doc` to open the local documentation in your browser.

Any time a type or function is provided by the standard library and you're not sure what it does or how to use it, use
the application programming interface (API) documentation to find out!

## Hello, World!

Now that you’ve installed Rust, it’s time to write your first Rust program.
It’s traditional when learning a new language to write a little program that
prints the text `Hello, world!` to the screen, so we’ll do the same here!

### Creating a Project Directory

You’ll start by making a directory to store your Rust code. It doesn’t matter to Rust where your code lives, but for the
exercises and projects in this book, we suggest making a projects directory in your home directory and keeping all your
projects there.
Open a terminal and enter the following commands to make a projects
directory and a directory for the “Hello, world!” project within the
projects directory.

```text
$ mkdir hello_world
$ cd hello_world
```

### Writing and Running a Rust Program

Next, make a new source file and call it `main.rs`. Rust files always end with the `.rs` extension. If you're using more
than one word in your filename, the convention is to use an underscore to separate them. For example,
use `hello_world.rs` rather than `helloworld.rs`.

```rust, editable
fn main() {
    println!("Hello, world!");
}
```

```text
$ rustc main.rs
$ ./main
Hello, world!
```

If `Hello, world!` did print, congratulations! You've officially written a Rust program. That makes you a Rust
programmer - welcome!

### Anatomy of a Rust Program

Let’s review this “Hello, world!” program in detail. Here’s the first piece of
the puzzle:

```rust, noplayground
fn main() {
#     println!("Hello, world!");
}
```

These lines define a function named `main`. The `main` function is special: it is always the first code that runs in
every executable Rust program. Here the first line declares a function named `main` that has no parameters and returns
nothing. If there were parameters, they would go inside the parentheses `()`.

The function body is wrapped in `{}`. Rust requires curly brackets around all function bodies. It's good style to place
the opening curly bracket on the same line as the function declaration, adding one space in between.

> If you want to stick to a standard style across Rust projects, you can use an automatic
> formatter tool called rustfmt to format your code in a particular style (more on rustfmt
> in Appendix D). The Rust team has included this tool with the standard Rust distribution, as rustc is, so it should
> already be installed on your computer!


The body of the `main` function hold the following code:

```rust, noplayground
# fn main() {
    println!("Hello, world!");
# }
```

This line does all the work in this little program: it prints text to the screen. There are four important details to
notice here.

First, Rust style is to indent with four spaces, not a tab.

Second, `println!` calls a Rust macro. If it had called a function instead, it would be entered as `println`(without
the `!`). We'll discuss Rust macros in more detail in Chapter 19. For now, you just need to know that using a `!` means
that you're calling a macro instead of a normal function and that macros don't always follow the same rules as
functions.

Third, you see the `"Hello, world!"` string. We pass this string as an argument to `println!`, and the string is printed
to the screen.

Fourth, we end the line with a semicolon(`;`), which indicates that this expression is over and the next one is ready to
begin. Most lines of Rust code end with a semicolon.

### Compiling and Running Are Separate Steps

You've just run a newly created program, so let's examine each step in the process.

Before running a Rust program, you must compile it using the Rust compiler by entering the `rustc` command and passing
it the name of your source file, like this:

```text
$ rustc main.rs
```

If you have a C or C++ background, you’ll notice that this is similar to
`gcc` or `clang`. After compiling successfully, Rust outputs a binary executable.

You can see the executable by entering the `ls` command in your shell:

```text
$ ls
main.exe  main.pdb  main.rs
```

This shows the source code file with the `.rs` extension, the executable file (`main.exe` on Windows, but `main` on all
other platforms), and, when using Windows, a file containing debugging information with the `.pdb` extension.

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else,
and they can run it even without having Rust installed.

Just compiling with `rustc` is fine for simple programs, but as your project grows, you'll want to manage all the
options and make it easy to share your code. Next, we'll introduce you to the Cargo tool, which will help you write
real-world Rust programs.

## Hello, Cargo!

Cargo is Rust’s build system and package manager. Most Rustaceans use
this tool to manage their Rust projects because Cargo handles a lot of tasks
for you, such as building your code, downloading the libraries your code
depends on, and building those libraries. (We call the libraries that your
code needs dependencies.)

The simplest Rust programs, like the one we’ve written so far, don’t have
any dependencies. If we had built the “Hello, world!” project with Cargo, it
would only use the part of Cargo that handles building your code. As you
write more complex Rust programs, you’ll add dependencies, and if you start
a project using Cargo, adding dependencies will be much easier to do.

Because the vast majority of Rust projects use Cargo, the rest of this
book assumes that you’re using Cargo too. Cargo comes installed with Rust
if you used the official installers discussed in “Installation” on page 1. If you installed Rust through some other
means, check whether Cargo is installed by entering the following in your terminal:

```text
$ cargo --version
```

### Creating a Project with Cargo

```text
$ cargo init
```

You'll see that Cargo has generated `Cargo.toml` file.

It has also initialized a new Git repository along with a `.gitignore` file. Git files won't be generated if you
run `cargo new` within an existing Git repository; you can override this behavior by using `cargo new --vcs=git`.

```text
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[[bin]]
name = "hello_world"
path = "main.rs"
```

This file is in the TOML(Tom's Obvious, Minimal Language) format, which is Cargo's configuration format.

The first line, `[package]`, is a section heading that indicates that the following statements are configuring a
package. As we add more information to this file we'll add other sections.

The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and
the edition of Rust to use.

The last line, `[dependencies]`, is the start of a section for you to list any of your project's dependencies. In Rust,
packages of code are referred to as `crates`. We won't need any other crates for this project, but we will in the first
project in Chapter 2, so we'll use this dependencies section then.

Cargo expects your source files to live inside the `src` directory. The top-level project directory is just for README
files, license information, configuration files, and anything else not related to your code. Using Cargo helps you
organize your projects. There's a place for everything, and everything is in its place.

### Building and Running a Cargo Project

```text
$ cargo build
```

This command creates an executable file in `target\debug\hello_cargo.exe` rather than in your current directory. Because
the default build is a debug build, Cargo puts the binary in a directory named `debug`. You can run the executable with
this command:

```text
$ ./target/debug/hello_world
Hello, world!
```

Running `cargo build` for the first time also causes Cargo to create a new file at the top level: `Cargo.lock`. This
file keeps track of the exact versions of dependencies in your project. This project doesn't have dependencies, so the
file is a bit sparse. You won't ever need to change this file manually; Cargo manages its contents for you.

We just built a project with cargo build and ran it with `./target/debug/
hello_world`, but we can also use `cargo run` to compile the code and then run
the resultant executable all in one command:

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\hello_world.exe`
Hello, world!
```

Using `cargo run` is more convenient that having to remember to run `cargo build` and then use the whole path to the
binary, so most developers use `cargo run`.

Notice that this time we didn't see output indicating that Cargo was compiling `hello_world`. Cargo figured out that the
files hadn't changed, so it didn't rebuild but just ran the binary. If you had modified your source code, Cargo would
have rebuilt the project before running it.

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but
doesn't produce an executable:

```text
$ cargo check
    Checking hello_world v0.1.0 (C:\Users\jerok\projects\study_blog\rust_lang_book\code\ch01\hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
```

Why would you not want an executable? Often, `cargo check` is much faster then `cargo build` because it skips the step
of producing an executable. If you're continually checking your work while writing the code, using `cargo check` will
speed up the process of letting you know if your project is still compiling! As such, many Rustaceans run `cargo check`
periodically as they write their program to make sure it compiles. Then they run `cargo build` when they're ready to use
the executable.

Let's recap what we've learned so far about Cargo:

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the `target/debug`
  directory.

An additional advantage of using Cargo is that the commands are the same no matter which operating system you're working
on. So, at this point, we'll no longer provide specific instructions for Linux and macOS versus Windows.

### Building for Release

When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations.
This command will create an executable in `target/release` instead of `target/debug`. The optimizations make your Rust
code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two
different profiles: one for development, when you want to rebuild quickly and often, and another for building the final
program you'll give to a user that won't be rebuilt repeatedly and that will run as fast as possible. If you're
benchmarking your code's running time, be sure to run `cargo build --release` and benchmark with the executable
in `target/release`.

### Cargo as Convention

With simple projects, Cargo doesn't provide a lot of value over just using `rustc`, but it will prove its worth as your
programs become more intricate.
Once programs grow to multiple files or need a dependency, it's much easier to let Cargo coordinate the build.

Even though the `hello_cargo` project is simple, it now uses much of the real tooling you'll use in the rest of your
Rust career. In fact, to work on any existing projects, you can use the following commands to check out the code using
Git, change to that project's directory, and build:

```text
git clone example.org/someproject
cd someproject
cargo build
```

## Summary

You're already off to a great start on your Rust journey! In this chapter, you've learned how to:

- Install the latest stable version of Rust using `rustup`
- Update to a newer Rust version
- Open locally installed documentation
- Write and run a "Hello, world!" program using `rustc` directly
- Create and run a new project using the conventions of Cargo

This is a great time to build a more substantial program to get used to reading and writing RUst code. So, in Chapter 2,
we'll build a guessing game program. If you would rather start by learning how common programming concepts work in Rust,
see Chapter 3 and then return to Chapter 2.
