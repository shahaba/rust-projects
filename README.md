# Rust Projects
![Rust workflow](https://github.com/shahaba/rust-projects/actions/workflows/rust.yml/badge.svg)

My working repo for going through the [Rust Book](https://rust-book.cs.brown.edu/title-page.html)

You can go into any of the folders and run: `cargo run` to compile and run the rust program (if you have `rustc` installed)

- [Rust Projects](#rust-projects)
  - [Chapter 2 - Guessing Game](#chapter-2---guessing-game)
  - [Chapter 3 - Variables and Controls](#chapter-3---variables-and-controls)
    - [Control Flow](#control-flow)
      - [`if` and `else if`](#if-and-else-if)
      - [`loop`, `while`, and `for`](#loop-while-and-for)
  - [Chapter 4 - Ownership](#chapter-4---ownership)

## Chapter 2 - Guessing Game

A quick dive into some of the core concepts in Rust like creating variables, loops, and accepting input from the command line (aka the user)

## Chapter 3 - Variables and Controls

Exploration of variable assignment, mutability, and other features like Shadowing that give a baseline understand of how to write simple programs with Rust. Also introduces function and control flow syntax:

### Control Flow

#### `if` and `else if`

A conditional expression that performs different actions depending on the value of a statement

Ex.

```Rust
let condition: bool = false;

if condition {
    println!("Hello World");
} else {
    println!("Nevermind");
}
```

**Note:** Unlike other languages, Rust won't convert non-booleans (like int: 0, 1) into booleans automatically. We need to explicitly pass a boolean to a `if` expression

#### `loop`, `while`, and `for`

- `loop`
  - Allows us to execute a block of code indefinitely, or until we explicitly tell it to stop
- `while`
  - Allows us to run a `loop`, while a given a condition is `true`
- `for`
  - Allows us to `loop` through a collection of elements

## Chapter 4 - Ownership

One of Rust's core concept to ensure programs run and use memory **safely**. Ownership is divided into three main principles: owning, referencing, and lifetimes.

1. Owning - Every value can only have a single owner at any given time.
   1. This deters race conditions that might allow for multiple alias of a variable to concurrently mutate the data.
   2. Ownership can be moved between variables, but there is always a single clear owner of the data.
2. Referencing (or Borrowing) - A **non-owning pointer** to data.
   1. Immutable (or Shared) Reference - created via `&` operator, allows us read access to data reference by pointer.
   2. Mutable (or Unique) Reference - created via `&mut` operator, allows us to mutate data referenced by pointer.
3. Lifetimes - Ensures the data that is being borrowed outlives its references.
