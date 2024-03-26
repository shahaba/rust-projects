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
  - [Chapter 5 - Structs](#chapter-5---structs)
    - [Defining Structs](#defining-structs)
    - [Struct update syntax](#struct-update-syntax)
    - [Struct Variations](#struct-variations)
    - [Methods](#methods)
  - [Chapter 6 - Enums](#chapter-6---enums)
    - [The Option Enum](#the-option-enum)
    - [The `match` Control Flow](#the-match-control-flow)
  - [Chapter 7 - Packages, Crates, and Modules](#chapter-7---packages-crates-and-modules)

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
   1. More in Chapter 10

## Chapter 5 - Structs

### Defining Structs

Like tuples, `struct`'s can be used to encapsulate multiple data types into a single variable. What separates the two is that structs requires parameters to be named. For example:

```Rust
struct User {
  username: String,
  email: String,
  active: bool,
  sign_in_count: u64
}
```

Structs are also entirely mutable; we can't mark individual fields as mutable, it has to be the whole struct.

### Struct update syntax

Structs can also be created from other structs (with all the Ownership rules still applying). The `..` syntax is used to specify remaining fields that haven't been set, taking values from another struct. The `..` must always come at the end of the init

```Rust
fn main() {
  // --snip-- 

  let user2 = User {
    email: String::from("another@example.com"),
        ..user1
  };
}
```

### Struct Variations

There are also two slight variations on the struct syntax:

- *Tuple Struct*
  - A struct with no named fields, used only to define types
    - Ex, Color: `struct Color(i32, i32, i32);` tuple struct
- *Unit-Like Struct*
  - A struct with no fields or names; Acts as a placeholder when we don't have any data to store as the type. This is convient for traits, which we see more in ch10
    - Ex.: `struct AlwaysEqual;`

### Methods

Structs can also implement methods or functions. They can be used to return a value or perform some operation with the struct, much like a `class` for an object in other languages

These methods are defined usings the `impl` keyword, outside and after the definition of a `struct`:

```Rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

We also see the use of an associated function (`square`) in the `impl` that allows us to create a new Rectangle of `square` shape via `let sq = Rectangle::square(3);`

We can also have more than 1 `impl` block for each struct

## Chapter 6 - Enums

Enums, or enumerations, allow us to define a type based on the different possible values it can take on. An enum can only be one of its values (or variants). For example, we can define a Enum for wether a value is of type V4 or V6 (different IP protocols):

```Rust
enum IpAddrKind {
    V4(String),
    V6(String),
}
```

Enums allow us to attach data to types directly, so we can set `IpAddrKind` V4 and V6 as both String types (or different types if we wanted to)

We can create a new variable that holds the enum type, as:

```Rust
// create a variable of enum type
let home = IpAddr::V4(String::from("127.0.0.1"));
```

### The Option Enum

Rust doesn't implement a `null` type like other languages. Instead, it uses an `Option` type to encode if a value is something or is nothing / absent (very similar to Null)

The reason for this is because trying to reference a value, when it's null, can lead to errors. It's a very easy error to make! It has more so to do with the implementation of nulls rather than the concept. The concept is actually quite useful.

The `Option` enum is so common that it's in the standard library, defined as:

```Rust
enum Option<T> {
	None,  // missing or absent value
	Some(T),
}
```

The `<T>` syntax is a generic type parameter that can be used to reference any primitive or type in Rust (more in Chapter 10)

### The `match` Control Flow

Like a switch statement in Scala, or a Case statement in SQL, Rust also implements a type of pattern matching code block that executes based on which pattern the input matches. For example, say we have an enum of Coins, each of a different type:

```Rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
```

We can write a `match` statement that allows us to execute a block of code based on which type of Coin is given

```Rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

The different patterns are referred to as `arms`, and consist of a pattern and some code. The `=>` operator separates the pattern from the code

**Note**: `match` operators are **exhaustive**, meaning they need to cover every possible outcome or type an enum can take. Rust implements a type of catch-all syntax, `other`, when we want to use the value in our code block (or `_` if we don't).

A toy example of `other` as a catch-all statement:

```Rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        other => 25,
    }
}
```

## Chapter 7 - Packages, Crates, and Modules
