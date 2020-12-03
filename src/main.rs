fn main() {
    //integer signed
    let x = 5;

    let x = x + 1;

    let x = x * 2;


    //integer unsigned
    let guess: u32 = "42".parse().expect("Not a number!");

    let guess = guess + 22;
    let guess = "6".parse::<u32>();

    //floating
    let x = 2.0;
    let y:f32 = 3.0;

    println!("The value of x is: {}", x);

    println!("{:?}", guess);

    //boolean
    let t = true;

    //char
    let c = 'c';

}
