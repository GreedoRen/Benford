fn main() {
    let s = String::from("hello");

    takes_onwnership(s);

    let x = 5;

    makes_copy(x);

    //println!("{}, {}", s, x); // x -> s is no longer viable
}

fn takes_onwnership(some_string: String) {
    println!("{}", some_string);
} 

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}