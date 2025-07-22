fn main() {
    let x = 20;
    // Declare a variable and initialize it with an if, else expression
    let y = if x % 2 == 0 { "even" } else { "odd" };

    println!("The number {x} is {y}");

    let mut a = 6;
    // Update a variable based on its value with an if, else expression
    a += if a > 5 { 2 } else { 1 };
    println!("The value is now {a}");

    is_divisible(x);
}

fn is_divisible(x: i32) {
    let div;

    if x % 4 == 0 {
        div = 4;
    } else if x % 3 == 0 {
        div = 3;
    } else if x % 2 == 0 {
        div = 2;
    } else {
        div = 1;
    }
    println!("The number {x} is divisible by {div}");
}