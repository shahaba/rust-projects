# Closures

Rust inherits some Functional Language features in the form of Closures, or anonymous functions. Very similar t a lambda function in python, we can use a closure to perform some mapping, filtering, etc.. operations on a sequence of data.

Unlike functions, closures are evaluated in the scope they're defined, and they can infer their data types from a given input. For example, the `unwrap_or_else` method can take a closure as an action on the `_or_else` side-effect (something we've already seen). This allows us to perform some custom actions when the `unwrap` returns an Error, like when a value does not exists.

```Rust
let s = Some(String::from("Hello World"));
let output = s.unwrap_or_else(|| String::from("Hello Universe"));

println!("{output}");
```

In this toy example, if we set s to `None`, the unwrap clause would call our closure and return `Hello Universe` instead!

Closures can also borrow, borrow mutably, or take ownership of a given input parameter. It depends on the what the body of the function does. This plays a large role when implement concurrent features, as we will see more in Ch16.
- We can always force ownership too with the `move` command

## Using Lifetimes

When we write functions that return closures, things get a little more complicated. Not only do we need to be aware of the different types of return closures, namely `Fn, FnOnce, FnMut`, but we also have to associate a Lifetime with the return value (because our reference can't outlive the data)
- `FnOnce` - Closure is called only once
- `FnMut` - Closure mutates values in its body, and can be called more than once
- `Fn` - Don't move or mutate values in the body, but also take nothing from their env

For example, a function that makes a clone of a string and returns the reference

```Rust
//              vvvv         vv                             vvvv                
fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    move || s_ref.to_string()
}
```

Where `'a` represents the input reference

# Iterators

We've seen iterators in earlier chapters as well, but here we add a little more details.

Iterators are lazy, and they don't take effect until we call a method that consumes the iterator. This can either be a reduce function, like `sum`, or using the `collect` function to consumer the iterator. The `for` loop also consumes an iterator, which is why we've never had to call `next` within a for loop!

```Rust
let v: Vec<i32> = vec![1, 2, 3, 4];

// Create iterator from v
let iterator = v.iter();

// Loop over values, mutiplying each by 2
let output = iterator.map(|x| x * 2).collect::<Vec<i32>>();

println!("{:?} {:?}", output, v);
```

All iterators implement the `Iterator` trait and the `next` method, which allows us to move through the iterator elements.
- `next` is called a consuming adaptor, because calling next allows us to use up the iterator

Other method that produce iterators, instead of consume them, are called iterator adaptors. For example, out `map` function is a iterator adaptor. We have to `collect` the values before the `map` function can be evaluated
