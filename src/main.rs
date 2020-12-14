fn main() {
    let s = String::from("hello world");

    let len = s.len();

    let hello = &s[0..5];
    let world = &s[6..11];
    let slice_start = &s[..4];
    let slice_end = &s[6..len];

    println!("{}, {}, {}, {}", hello, world, slice_start, slice_end);
}
