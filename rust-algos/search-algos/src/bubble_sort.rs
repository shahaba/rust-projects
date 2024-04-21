use rand::Rng;

pub fn bubble_sort(vec: &mut Vec<i32>) {
    let mut left = 0;
    let n = vec.len();

    while left < n {
        for i in 1..n {
            if vec[i - 1] > vec[i] {
                vec.swap(i - 1, i)
            }
        }

        left += 1;
    }
}

pub fn optimized_bubble_sort(vec: &mut Vec<i32>) {
    let mut n = vec.len();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 1..n {
            if vec[i - 1] > vec[i] {
                vec.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }
}

pub fn check_sorted(vec: &Vec<i32>) -> bool {
    for index in 1..vec.len() {
        if vec[index - 1] > vec[index] {
            return false;
        }
    }
    true
}

pub fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);

    for _ in 0..num_items {
        vec.push(rng.gen_range(0..max));
    }

    vec
}

#[cfg(test)]
mod test {
    use crate::bubble_sort::{bubble_sort, check_sorted, make_random_vec, optimized_bubble_sort};

    #[test]
    fn test_random_list() {
        let mut arr = make_random_vec(10, 30);
        bubble_sort(&mut arr);
        assert!(check_sorted(&arr))
    }

    #[test]
    fn test_reversed_list() {
        let mut arr = vec![4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert!(check_sorted(&arr))
    }

    #[test]
    fn test_random_list_optimized() {
        let mut arr = make_random_vec(10, 30);
        optimized_bubble_sort(&mut arr);
        assert!(check_sorted(&arr))
    }

    #[test]
    fn test_reversed_list_optimized() {
        let mut arr = vec![4, 3, 2, 1];
        optimized_bubble_sort(&mut arr);
        assert!(check_sorted(&arr))
    }
}
