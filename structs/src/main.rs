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
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
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
