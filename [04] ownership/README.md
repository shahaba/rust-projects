# Chapter 4 - Ownership

One of Rust's core concept to ensure programs run and use memory **safely**. Ownership is divided into three main principles: owning, referencing, and lifetimes.

1. Owning - Every value can only have a single owner at any given time.
   1. This deters race conditions that might allow for multiple alias of a variable to concurrently mutate the data.
   2. Ownership can be moved between variables, but there is always a single clear owner of the data.
2. Referencing (or Borrowing) - A **non-owning pointer** to data.
   1. Immutable (or Shared) Reference - created via `&` operator, allows us read access to data reference by pointer.
   2. Mutable (or Unique) Reference - created via `&mut` operator, allows us to mutate data referenced by pointer.
3. Lifetimes - Ensures the data that is being borrowed outlives its references.
   1. More in Chapter 10
