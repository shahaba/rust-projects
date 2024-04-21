use rand::Rng;

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
