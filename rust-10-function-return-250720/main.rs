
fn main() {
    x = 10;
    y = plus_one(x);
    println!("x was {x}, now y is {y}");
}

fn plus_one(x: i32) -> i32 {
    x++
}
