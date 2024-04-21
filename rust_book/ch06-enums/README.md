
# Chapter 6 - Enums

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

## The Option Enum

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

## The `match` Control Flow

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
