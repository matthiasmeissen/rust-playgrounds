// In addition to variables that are immutable by default
// There are also const which are always immutable
// The type of the value must always specified
// And by convention they are written in UPPERCASE_WITH_UNDERSCORES
// They are helpful to specify values for the whole program in one central place

// Difference between let and const
// const is evaluated at compile time and stores a literal value, so it never exists as variable in a program
// let without mut is evaluated at runtime, 
// For let x = 20, this means 20 is stored at a specific location in memory at runtime
// And x is a binding to that location

const HOURS_IN_SECONDS: u32 = 60 * 60;

fn main() {
    // Declare immutable variable (default) and initialize with 20
    let x = 20;
    // Use println!() macro to print value to console
    println!("The value of x is: {x}.");

    // Declare mutable variable and initialize with 10
    let mut y = 10;
    // Print value of y
    println!("The value of y is: {y}");

    // Assing new value to mutable variable and print
    y = 40;
    println!("The new value of y is: {y}");

    println!("One hour contains {HOURS_IN_SECONDS} seconds.");
}