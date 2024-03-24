fn main() {
    // Create a heap for array [0; 10]
    let a = Box::new([0; 10]);
    println!("a: {:?}", a);
    // Change ownership of pointer to array from a --> b
    let b = a;
    println!("New owner b: {:?}", b);

    // Managing ownership of `first`
    let first = String::from("Micheal");
    let first_clone = first.clone();

    // Mutate first, creating a new reference to string
    let first_suffix = add_suffix(first);
    println!("{first_suffix} - Originally {first_clone}");

    // What about passing arguments but not losing the ownership?
    // This is where references
    let m1: String = String::from("hello");
    let m2: String = String::from("world");
    reference(&m1, &m2);

    println!("{m1} {m2}");

    let mut x: i32 = 4;
    println!("First value of x: {x}");

    x = borrowing(&mut x);
    println!("Mutated value of x from borrowing: {x}");

    // using slices to interact with Strings
    let s: String = String::from("HelloWorld Universe");
    let slice = first_word(&s);

    println!("First Word of {}: {} and {}", s, first_word(&s), slice);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn reference(g1: &String, g2: &String) {
    println!("{g1} {g2} in a function")
}

fn borrowing(x: &mut i32) -> i32 {
    let mut v: Vec<i32> = vec![1, 2, 3];

    println!("Vector v: {v:?}");

    let num: &i32 = &v[2];

    println!("The 3rd number in v: {num}");

    v.push(4);

    // can't change v because num is still in use, and we would be changing the reference
    // println!("The 3rd number in v: {num}"); // would fail

    *x + 1
}

fn first_word(word: &String) -> &str {
    let bytes = word.as_bytes();

    // iterate through word
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &word[0..i];
        }
    }

    &word[..]
}
