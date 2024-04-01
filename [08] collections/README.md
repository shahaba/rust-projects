## Chapter 8 - Collections

In Rust, collections are data structures that can hold multiple values, similar to arrays or tuples in other languages. Collections hold their data in the heap, allowing the size of the data structure to grow and shrink as the program needs

There are 3 main types of collections discussed in this chapter:

- **Vectors**: (similar to dynamic arrays) can store a variable number of values in a single data structure
- **Strings**: a collection of characters, the `String` type we interact with frequently
- **HashMaps**: a key-value pair mapping for speedy lookup of data. In rust, this is an more general implementation of a map

## Storing Values in Vectors

The syntax for vectors is `Vec<T>`, where T is a placeholder for a particular type. Vectors hold a variable number of values right next to each other in memory, but it can **only store values of the same type**. Vectors are smart in that they can also infer the data types from a given input (as supposed to defining the type, as below)

Example:

- `let v: Vec<i32> = Vec::new();`  Creates a new empty vector of type i32
- `let v = vec![1, 2, 3, 4];` Create a new vector from a list of value, type inferred

### Reading a Vector

We can reference values in a vector in two ways, either via indexing (square brackets) or using the `get` method

- If we reference an out-of-bound index, using indexing, it will cause our program to panic (as expected)
- If we instead use the `get` method, it returns an `Option<T>` value from which we can use the `None` value to reference out-of-bounds indexes

Example:

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

### Inserting Values into a Vector

We can use the `push` method to insert new values into our vector

### Using Enums with Vectors

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

## Strings

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

## HashMaps

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
