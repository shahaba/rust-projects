# Chapter 7 - Packages, Crates, and Modules

Packages, crates, and modules in Rust allow us to organize our code, keep certain details private while exposing others. Collectively, they are referred to as the module system that includes:

- **Package**: A cargo feature for building, testing, and sharing crates
- **Crate**: A tree of modules that produce and executable
- **Modules** and the **use** statement: Allow us to control the scope and access to paths
- **Paths**: A way of naming items, like structs, functions, or modules

## What is a Crate?

A crate is the smallest amount of code the compiler with consider at runtime. It can contain modules, which can be defined in other places in our project, that get compiled into a crate

A crate can either be a:

- **Binary Crate**: programs that compile into an executable. Each crate must contain one, and only one, `main` function as its entry-point
- **Library Crate**: don't have main functions, and don't compile into an executable. Rather, they define features that can be shared with other projects

The crate root is the source file from which the compiler starts file tree that governs our crate

## What is a Package

A package is a bundle of one of more crates that provide us with a set of functionality. It contains a cargo.toml file that describes how to build those crates. Cargo is actually a package itself, that compiles our binary crates in an executable. It also has library crates which the binary crate depends on, that can also be shared with other programs

`main.rs` is always interpreted as the crate root of a binary crate with the same name as the package. If, instead, we had a `src/lib.rs` file, our package contains a library crate with the same name as the package, and it's the crate root.

## Defining a Module

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
