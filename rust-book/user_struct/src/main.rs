struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user1 = User {
        email: String::from("test@example.com"),
        username: String::from("test user"),
        sign_in_count: 120,
        active: false,
    };
    user1.email = String::from("hoge@example.com");
}

fn build_user1(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        sign_in_count: 120,
        active: true
    }
}

// init shorthand.
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 120,
        active: true,
    }
}
