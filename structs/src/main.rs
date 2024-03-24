struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// add debug trait to struct to print in console
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// implement a method on Rectangle struct
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("John Doe"),
        active: true,
        sign_in_count: 1
    };
    
    println!("User1: {:#?}", user1.email);

    let user2: User = build_user("else@example.com".to_string(), "Jane Doe".to_string());

    // println!("User2 Name: {:#?}", user2.email);

    let mut rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("The area of the rectangle is {} square pixels", area(&rect));
    println!("The area struct is: {:#?}", rect);
    println!("Using built-in method for area {}", rect.area());
    println!("Using built-in method for if width > 0 {}", rect.width());
    println!(
        "Using built-in method for if rect1 can hold rect2 {}",
        rect.can_hold(&rect2)
    );

    rect.set_width(20);
}

// declared using field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// area function that consumes a struct
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}