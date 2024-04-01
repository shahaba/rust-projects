
# Chapter 11 - Automated Tests

In Rust, tests are organized into two main categories: **unit tests** and **integration tests**

- **Unit Tests**
  - Embedded into our `src/lib.rs` directly, they're meant to be small focused tests for a single module
    - Convention is to create a `mode test {...}` in the file that contains the tests we want
  - Has access to private function, being in the same file as the library
  - Starts with `#[cfg(test)]` which tells rust to ignore tests unless explicitly called via the `cargo test` (i.e. `cargo run` or `cargo build` do not run theses tests)
    - This saves time on compile, so we don't load our library with tests
    - Each test is labeled with a `#[test]`, and written as a `fn`
    - `cfg` stands for configuration

```Rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
```

- **Integration Tests**
  - Integration tests are stored externally to our library (unlike Unit Tests)
    - This allows us to bring in multiple parts of our library's public API, and test if they all work together as designed
    - Stored in a `tests/` directory
  - Each file in the `tests` directory acts like a separate crate, and we need to bring all the dependencies into scope for each crate

## Using `assert!` Macros

Rust provides an `asset!` macro that helps standardize conditions that we want to evaluate to `true`

- This is the same as using a block `if` statement to check if output is `true`

Rust also takes very common tests like asserting if two values are equal or not equal, and packaged them into macros: `assert_eq!` and `assert_neq!`

Each macro can also take a custom message, so that we can add more details to on failure:
`assert_eq!(left_result, right_result, custom_message);`
