fn main() {
   let user1 = User {
       email: String::from("some_emain@gmail.com"),
       username: String::from("some_username"),
       active: true,
       sign_in_count: 1,
   };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}