
# Ch15 - Smart Pointers


Smart pointers are data structure that behave like pointers (references) but come with additional metadata and capabilities that help us adapt to specific scenarios

  - The main difference between a reference and a smart pointer is that the smart pointer will also own the data it points to
	  - For example, `Strings` and `Vec<T>` are take advantage of smart pointer like features, but are still classified as collections
  - Smart Pointers are built as `structs`.  They implement the `Deref` and `Drop` traits that allow us to control reference behaviours
	  - `Deref` = allows an instance of a struct smart pointer to behave like a normal reference
	  - `Drop` = allows us to customize how code runs when our smart pointer goes out of scope

In the standard library, there are 3 common smart pointers we can use (alongside creating our own):
  
  - `Box<T>` = for storing values on the heap, mutable or immutable
  - `Rc<T>` = for reference counting for immutable borrows with multiple owners
  - `Ref<T>` and `RefMut<T>` , via `RefCell<T>` , that allows us to delay borrowing rules to runtime instead of compile time
	  - This allows us to do certain actions that would not be allow by the borrow checker at compile time

## Using `Box<T>`

The simplest smart \pointer, `Box<T>` allows us to point to data on the heap rather than on the stack, with just a pointer to the heap data stored on the stack

Example Box:

```Rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

The downside to a Box data structure is that it is single ownership based, so we end up transferring ownership on reference.

## Using `Rc<T>`

Also called reference counting, it's an extension to storing data on the heap, `Rc<T>` , allows us to have multiple (immutable) references and keeps track of each. When there are no more references to a value, it can clean up the smart pointer without any references being lost.

However, like `Box` , these are single-thread data structures that shouldn't be used in multi-threaded contexts. 

## Using `RefCell<T>`

While `Rc<T>` gives us an immutable reference, there are times when we would want to use mutable ones to internal data. This is called interior mutability, which allows us to mutate data even if there are immutable references to that data (something not typically allows with the borrow checker)

# Summary

How do we go about choosing between `Box`, `Rc`, and `RefCell`?

- `Rc` allows us to have multiple owners of the same data; `Box` and `RefCell` only allow single ownership
- `Box` allows immutable or mutable borrows, checked at runtime; `Rc` does immutable borrow checked at compile time; `RefCell` allows immutable or mutable borrow checked at runtime
	- `RefCell` allows us to mutate values inside the data structure even when it is immutable

The downside to `Rc` and `RefCell` is that we can create Memory Leaks, where memory is created but never cleaned up.
