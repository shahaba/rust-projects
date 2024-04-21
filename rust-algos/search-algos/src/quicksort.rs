pub fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let p = partition(arr);

    quicksort(&mut arr[0..p]);
    quicksort(&mut arr[p + 1..]);
}

pub fn partition(arr: &mut [i32]) -> usize {
    let n = arr.len() - 1;
    let pivot = arr[n];

    let mut i = 0;

    for j in 0..n {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, n);
    i
}

#[cfg(test)]
mod test {
    use crate::helpers::{check_sorted, make_random_vec};
    use crate::quicksort::quicksort;

    #[test]
    fn test_random_quicksort() {
        let mut arr = make_random_vec(30, 10000);
        quicksort(&mut arr[..]);
        assert!(check_sorted(&arr));
    }
}
