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

Packages, crates, and modules in Rust allow us to organize our code, keep certain details private while exposing others. Collectively, they are referred to as the module system that inclues:

- **Package**: A cargo feature for building, testing, and sharing crates
- **Crate**: A tree of modules that produce and executable
- **Modules** and the **use** statement: Allow us to control the scope and access to paths
- **Paths**: A way of naming items, like structs, functions, or modules

### What is a Crate?

A crate is the smallest amount of code the compiler with consider at runtime. It can contain modules, which can be defined in other places in our project, that get compiled into a crate

A crate can either be a:

- **Binary Crate**: programs that compile into an executable. Each crate must contain one, and only one, `main` function as its entry-point
- **Library Crate**: don't have main functions, and don't compile into an executable. Rather, they define features that can be shared with other projects

The crate root is the source file from which the compiler starts file tree that governs our crate

### What is a Package

A package is a bundle of one of more crates that provide us with a set of functionality. It contains a cargo.toml file that describes how to build those crates. Cargo is actually a package itself, that compiles our binary crates in an executable. It also has library crates which the binary crate depends on, that can also be shared with other programs

`main.rs` is always interpreted as the crate root of a binary crate with the same name as the package. If, instead, we had a `src/lib.rs` file, our package contains a library crate with the same name as the package, and it's the crate root.

### Defining a Module

- **Declaring Modules**: In the root of the crate, we can declare new modules. The compiler will then look for the definition of the crate in the following patter:
  - Inline, within the curly brackets that follow the `mod` keyword
  - In a file named after the module
  - In a file under the module folder named `mods.rs`

For example, if we declared a `garden` module with `mod garden;`, we would do one of the above to define the code associated with our module

- **Declaring submodules**: We can declare submodules in any other crate other than the root. For example, if we had a file for our garden module, `src/garden.rs`, we can declare a vegetables submodule as `mod vegetables;`
  - This submodule can be stored in a separate file or directly in the `garden.rs` file

- **Private vs. Public**: By default, any code not in the crate root is private. We have to explicitly declare a module as public, with the `pub` keyword, if we want to access it in other parts of our program. This applies to modules (`pub mod`), to structs (`pub struct`), to enums (`pub enum`), and to functions (`pub fn`)
  - Fields inside of a struct are also private by default and need to be tagged with `pub` if we want to access them directly outside of the module file
  - `impl` don't need the `pub` tag, but their functions do

- **`use` Keyword**: When referring to a module outside of its declaration file, we have to reference it by its full path. For example, if we had a `struct Asparagus { }` in our vegetables.rs, we would need to reference it as: `crate::garden::vegetables::Asparagus;`. If we use the keyword `use crate::garden::vegetables::Asparagus` , we can instead call `Asparagus { }` directly
  - This saves us on the verbose declaration of submodules

### Chapter 8 - Collections

In Rust, collections are data structures that can hold multiple values, similar to arrays or tuples in other languages. Collections hold their data in the heap, allowing the size of the data structure to grow and shrink as the program needs

There are 3 main types of collections discussed in this chapter:
- **Vectors**: (similar to dynamic arrays) can store a variable number of values in a single data structure
- **Strings**: a collection of characters, the `String` type we interact with frequently
- **HashMaps**: a key-value pair mapping for speedy lookup of data. In rust, this is an more general implementation of a map

### Storing Values in Vectors

The syntax for vectors is `Vec<T>`, where T is a placeholder for a particular type. Vectors hold a variable number of values right next to each other in memory, but it can **only store values of the same type**. Vectors are smart in that they can also infer the data types from a given input (as supposed to defining the type, as below)

Ex. 
- `let v: Vec<i32> = Vec::new();`  Creates a new empty vector of type i32
- `let v = vec![1, 2, 3, 4];` Create a new vector from a list of value, type infered

#### Reading a Vector

We can reference values in a vector in two ways, either via indexing (square brackets) or using the `get` method
- If we reference an out-of-bound index, using indexing, it will cause our program to panic (as expected)
- If we instead use the `get` method, it returns an `Option<T>` value from which we can use the `None` value to reference out-of-bounds indexes

Ex.
- Indexing: `let y = v[0];` - reference but not taking ownership add `&`
- `get` method: `let y = v.get(0)`

We can also iterate and enumerate our way through a vector, as below:

```Rust
// This program panics
fn main(){
	let v: Vec<i32> = vec![1, 2, 3, 4];
	for n_ref in v.iter() {
	    println!("{n_ref}");
    }
}
```

**Note**: `iter` takes a immutable reference to `v` or the vector, and we can't modify that vector within the loop (otherwise it will deallocate the previous pointer)

#### Inserting Values into a Vector

We can use the `push` method to insert new values into our vector

#### Using Enums with Vectors

Sometimes storing the same type in vectors can be inconvenient at times. Luckily there's a workaround with enums. As enums are defined all as the same type, we can easily return different types under the enum variants!

Example:

```Rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

```

### Strings

The `String` type is provided with the Rust standard library, different to the built-in string type `str` slice we usually see in its borrowed form `&str`

The `String` type stores its data in the heap, and so is growable, mutable, and owned
- Strings are actually implement as a wrapper around a vector of bytes that come with some extra guarantees, restrictions, and capabilities
	- We can push more data onto the String, we can convert a `"String"` into a String type vector using the `to_string` method, and it implements the `Display` trait
	- `push_str` take a string slice, because we don't necessarily want to take ownership of the str we're pushing
- We can concatenate strings with the `+` operator or the `format!` macro
- Strings don't support indexing, instead we need to iterate over a sequence of chars using the `chars` method
	- This is because the way strings are stored as vectors of bytes, a single character may not map to a single byte in the vector

Example: Iterating over a String

```Rust
for c in "Hello".chars() {
    println!("{c}");
}
```

One thing to note about String is how they are encoded. Strings in Rust use the UTF-8 encoding, and that's why reading the byte vector may not always return the value we expect from a string
### HashMaps

The last common collection in Rust is the `HashMap<K, V>`, which stores a mapping of the key `K` to a value `V` using a hashing function. Similar to a dictionary in Python

```Rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

When inserting values into a hash map, if the type implements the Copy trait its values are copied directly, otherwise (like String) it takes ownership

HashMaps also come with methods to surface capabilities we find Python, like inserting a new value if the key doesn't exist or updating its value if it does.
- This is done via the `entry` api that checks the key we give it, and the return value is an enum called Entry that may or may not exist
- Using the `or_insert` , we can check if the key exists, and if not, insert the value with the given key

## Chapter 9 - Error handling

In Rust, there are 2 ways to handle errors:
- **Recoverable**
	- When we want to report back to the user that the operation failed but they can continue to use the program and retry the operation
	- Instead of ending the program, Rust can return a `Result` enum with the variants `O` and `Err`
		- This allows us to customize the actions or returns upon failure

The Result Enum:
```Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

which allows us to write programs like the following that return an Error if we can't find a file called "hello.txt"
```Rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

We can even take this a step further and `match` different types of Errors:

```Rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```


- **Unrecoverable**
	- When we want the program to exit, as the result of a bug or trying to access a value in beyond the end of an array
		- This is usually done with a `panic!` macro
	- By default, `panic` causes our program to unwind. That is, Rust walks back up the stack and cleans up the data for each function. This can be a lot of work, so there is also the alternative of aborting immediately under a panic which just ends the program without cleaning up
		- Done by setting `panic = 'abort'` in the `[profile.release]` of our toml
	- There are shortcuts for calling Panic on an Error
		- `unwrap`
			- Shortcuts the match statement by returning the result or calls panic if it Err's
		- `expect`
			- Similar to unwrap, except we can choose the output of the error message

### Propagating Error

Writing match statements that Err when something fails is very common in Rust. Instead of handling the error in the function, we can actually return it to the calling code and let it decide what to do. This is called *propagating errors*

Example:
```Rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

This is so common, there's actually a shortcut operation called `?` to make this simpler. Rewritting the above code, gives us:

```Rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

**Note**: `?` can only be used in functions whose return type is compatible with `?` (that is Ok and Err `Results`, `Options`, or another implementation of `FromResidual`)

