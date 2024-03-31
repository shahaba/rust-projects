use std::{cmp::PartialOrd, fmt::Display};

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    println!("Hello, world!");

    // find the largest integer using largest_i32
    let vec = vec![1, 5, 100, 2, 10];

    println!("Largest value is: {}", largest_i32(&vec));

    // using our generic function 
    println!("Largest generic value is: {}", largest(&vec));

    let pt = Point { x: 1, y: 10 };

    println!("{:?}", pt);

    let pair = Pair::new(1, 10);
    pair.cmp_display();
}

// Using generics instead of types as input params
// we need to restrict T to being comaparable 
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// Using explicit types as parameters to functions
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Traits
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
