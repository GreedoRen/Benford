fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    let guess: u32 = "42".parse().expect("Not a number!");

    let guess = guess + 22;

    let guess = "4".parse::<u32>();
    let guess = "6".parse::<u32>();

    println!("The value of x is: {}", x);

    println!("{:?}", guess);
}
