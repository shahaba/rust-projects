// An enum representing an ip address
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// An enum with different types
// instead of having multiple structs, we can have just one enum
#[derive(Debug)]
enum Message {
    // Quit,
    // Move { x: i32, y: i32 },
    Write(String),
    // ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Message {
    fn call(&self) {
        println!("Message: {:?}", &self);
    }
}

fn main() {

    // 
    let home = IpAddr::V4(127, 0, 0, 1);

    println!("Home IP Address: {:?}", home);

    let loopback = IpAddr::V6(String::from("::1"));
    println!("Loopback IP Address: {:?}", loopback);

    let m = Message::Write(String::from("Hello"));
    m.call();

    let penny = Coin::Penny;
    println!("Value in Cents: {}", value_in_cents(penny));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Five {:?}, Six {:?}, None {:?}", five, six, none);

    let opt: Option<String> = Some(String::from("Hello World"));

    match opt {
        Some(_) => println!("Some!"),
        None => println!("None!"),
    }

    println!("{:?}", opt);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}