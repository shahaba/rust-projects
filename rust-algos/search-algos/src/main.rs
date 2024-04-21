pub mod bubble_sort;

use bubble_sort::bubble_sort;

fn main() {
    println!("Hello, world!");
    let mut arr = vec![1, 5, 3, 4];
    bubble_sort(&mut arr);

    println!("{:?}", arr);
}
