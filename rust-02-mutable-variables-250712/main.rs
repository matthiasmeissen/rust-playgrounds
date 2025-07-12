
// To compile this program use: rustc <programName>

// When you declare a new variable with let it is immutable
// This means you can not change it later on (similar to a const)
// The compiler will not compile when you do so

// To make a variable mutable you need to specify this with "mut"
// let mut x = 5 creates a mutable variable

fn main() {
    // This creates a variable and initializes it to 5
    // By default variables are immutable
    let x = 5;
    // Being immutable means they are not allowed to change
    // This is why the following line is commented out, since it won't compile
    // x = 10;

    // Print the value with println! macro
    println!("The value of x is: {x}.");

    // Declare a mutable variable and initialize with 20
    let mut y = 20;
    println!("The value of y is: {y}.");

    // Assing a new value to y, which is possible sine it is a mutable variable
    y = 40;
    println!("The value of y is now: {y}.");
}
