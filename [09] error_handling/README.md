# Chapter 9 - Error handling

In Rust, there are 2 ways to handle errors:

- **Recoverable**
 . - When we want to report back to the user that the operation failed but they can continue to use the program and retry the operation
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

## Propagating Error

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

This is so common, there's actually a shortcut operation called `?` to make this simpler. Rewriting the above code, gives us:

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
