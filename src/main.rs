fn main() {
   let x = five();
   let y = plus_one(1);

   println!("{}", x);
   println!("{}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(y:i32) -> i32 {
    y + 1
}