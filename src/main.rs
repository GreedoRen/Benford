fn main() {
    let user1 = User {
        username: String::from("some_username"),
        email: String::from("some_email@gmail.com"),
        sign_in_count: 1,
        active: true,
    }

    let user2 = User {
        username: String::from("anoter_username"),
        email: String::from("another_email@gmail.com"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
