fn main() {
    let user1 = build_user("thawindu126@gmail.com", "thawindu126");
    let user2 = User {
        username: String::from("at19"),
        email: String::from("angeshtuto@icloud.com"),
        active: false,
        ..user1
    };

    println!("username: {}", user2.username);
    println!("email: {}", user2.email);
    println!("sign_in_count: {}", user2.sign_in_count);
    println!("active: {}", user2.active);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: email.to_string(),
        username: username.to_string(),
        active: true,
        sign_in_count: 1,
    }
}
