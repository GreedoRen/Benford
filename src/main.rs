fn main() {
    let user1 = User {
        username: String::from("some_username"),
        email: String::from("some_email@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        username: String::from("anoter_username"),
        email: String::from("another_email@gmail.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(-10, -10, -10);
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

struct Color(u32, u32, u32);
struct Point(i32, i32, i32);

