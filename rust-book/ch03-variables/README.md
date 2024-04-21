# Chapter 3 - Variables and Controls

Exploration of variable assignment, mutability, and other features like Shadowing that give a baseline understand of how to write simple programs with Rust. Also introduces function and control flow syntax:

## Control Flow

### `if` and `else if`

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

### `loop`, `while`, and `for`

- `loop`
  - Allows us to execute a block of code indefinitely, or until we explicitly tell it to stop
- `while`
  - Allows us to run a `loop`, while a given a condition is `true`
- `for`
  - Allows us to `loop` through a collection of elements
