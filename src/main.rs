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
    let y: f32 = 3.0;

    println!("The value of x is: {} {}", x, y);

    println!("{:?}", guess);

    //boolean
    let t = true;

    //char
    let c = 'c';
    let cat = '😻';

    //turple
    let tup: (i32, f64, char) = (2, 5.1, 'A');
    let (x, y, z) = tup;
    print!("The value of x, y, z is: {}, {}, {}", x, y, z);

    //array
    let array:[i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]
}
