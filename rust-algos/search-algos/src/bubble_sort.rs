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

#[cfg(test)]
mod test {
    use crate::bubble_sort::{bubble_sort, optimized_bubble_sort};
    use crate::helpers::{check_sorted, make_random_vec};

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
