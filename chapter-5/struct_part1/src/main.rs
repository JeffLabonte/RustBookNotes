struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user1 = User {
        username: String::from("someGuy"),
        email: String::from("someGuy@example.com"),
        sign_in_count: 3000,
        active: true
    };

    user1.email = String::from("otherGuy@example.com");
}
