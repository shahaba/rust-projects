use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();

    v.push(5);

    println!("What's on V: {:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("Third value: {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // let mut v = vec![1, 2, 3];
    // dup_in_place(&mut v);
    //

    let mut s = String::new();
    println!("{s}");

    let data = "initial comment";
    let s = data.to_string();

    println!("{s}");

    let mut s = String::from("initial value");
    s.push_str(" baar");

    println!("{s}");

    // Hash Maps
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Retrieved Score: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.entry(String::from("Yellow")).or_insert(50);

    println!("{:?}", scores);

    let mut v: Vec<i32> = vec![1, 2];

    for mut n_ref in 0 .. v.len() {
        n_ref += 1

        println!("Modified value: {}", n_ref);
    }  
}

// Running this function would cause Rust to panic
fn dup_in_place(v: &mut Vec<i32>){
  // we can't mutate v when we use an iterator
    // because Rust will deallocate the old reference when we add new data
    for n_ref in v.iter(){
        v.push(*n_ref); // duplicate the value in place
    }
}
