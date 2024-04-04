
# Chapter 5 - Structs

## Defining Structs

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

## Struct update syntax

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

## Struct Variations

There are also two slight variations on the struct syntax:

- *Tuple Struct*
  - A struct with no named fields, used only to define types
    - Ex, Color: `struct Color(i32, i32, i32);` tuple struct
- *Unit-Like Struct*
  - A struct with no fields or names; Acts as a placeholder when we don't have any data to store as the type. This is convinient for traits, which we see more in ch10
    - Ex.: `struct AlwaysEqual;`

## Methods

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
