fn main() {
   let x = five();
   let y = plus_one(1);
   let z = two();
   let n = minus_three(1);

   println!("{}", x);
   println!("{}", y);
   println!("{}", z);
   println!("{}", n);
}

fn five() -> i32 {
    5
}

fn plus_one(y:i32) -> i32 {
    y + 1
}

fn two() -> u32 {
    2
}

fn minus_three(n:i32) -> i32 {
    n - 3
}