
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    variables();
    another_function(5);
    println!("Expression: {}", expression());
    branches();
}

fn variables() {
    println!("3hrs in seconds: {THREE_HOURS_IN_SECONDS}");

    let mut x:u8 = 5;
    println!("The value of x is: {x}");
    
    x = 6;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");

    // floating point number
    let x: f32 = 3.0;
    println!("The value of x is: {x}");

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 5);

    let var = tup.0;
    println!("The first value of tup is: {var}");

    // destructuring
    let (x, y, z) = tup;
    println!("The first value of tup is: {x}");
    println!("The second value of tup is: {y}");
    println!("The third value of tup is: {z}");

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
}

fn another_function(x: i32) {
    println!("Another message, the value of x is: {x}")
}

fn expression() -> i32 {
    let y = 5;
    return y + 1
}

fn branches() {
    let number = 5;

    if number < 5 {
        println!("condition was true")
    } else if number > 5 && number < 10 {
        println!("condition was false")
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number was: {number}");

    // a casual infinite loop without break
    loop {
        println!("again!");
        break println!("Get me out of here");
    }

    let mut counter: i8 = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result was: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}