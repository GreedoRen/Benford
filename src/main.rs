fn main() {
   let mut user1 = User {
       email: String::from("some_email@gmail.com"),
       username: String::from("some_username"),
       active: true,
       sign_in_count: 1,
   };

   user1.email = String::from("second_email@gmail.com");
   
   println!("{}", user1.email); 
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}