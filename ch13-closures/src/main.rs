fn main() {
    println!("Hello, world!");

    let s = None; // Some(String::from("Hello World"));
    let output = s.unwrap_or_else(|| String::from("Hello Universe"));

    println!("{output}");

    // Mutable Closure
    let mut v = vec![1, 2, 3, 4];
    let mut borrows_mutably = || v.push(10);
    borrows_mutably();

    // Using a for loop as consuming adaptor (or calls to .next())
    for value in [1, 2, 3, 4] {
        println!("{}", value * 2);
    }

    // Using iterator adaptor to manipulate values
    let v: Vec<i32> = vec![1, 2, 3, 4];
    let iterator = v.iter();

    println!("{:?}", v);

    let output = iterator.map(|x| x * 2).collect::<Vec<i32>>();

    println!("{:?} {:?}", output, v);
}
