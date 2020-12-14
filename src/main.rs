fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11]; 
    let slice = &s[..4];

    println!("{}, {}, {}", hello, world, slice);
}
