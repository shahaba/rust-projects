pub mod bubble_sort;
pub mod helpers;
pub mod quicksort;

use bubble_sort::bubble_sort;
use quicksort::quicksort;

fn main() {
    println!("Hello, world!");
    let mut arr = vec![1, 5, 3, 4];
    bubble_sort(&mut arr);

    println!("Bubble Sort: {:?}", arr);

    let mut arr = vec![1, 5, 3, 4];
    quicksort(&mut arr[..]);
    println!("Quicksort: {:?}", arr);
}
