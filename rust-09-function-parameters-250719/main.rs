fn main() {
    // It seems that you can call a function before its declaration (no need for function prototypes)
    // Then we can call the function with our own arguments
    print_value(20);

    measure_values(100, 'm');
}

// Function names need to be snake_case (as variable names should as well)
// You need to specify the type for variable paramaters
fn print_value(x: i32) {
    println!("Your value is: {x}");
}

// When you want more then one argument in a function you add a comma to separate them
fn measure_values(val: i32, unit: char) {
    println!("The space is {val} {unit}");
}