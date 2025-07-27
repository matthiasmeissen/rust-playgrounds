fn main() {
    // Declare a immutable variable and initialize to 20.0 (which creates a 64 bit float)
    let f = 20.0;
    // Declare immutable variable and initialize with return value from function
    let c = fahrenheit_to_celsius(f);
    // Print both values
    println!("The temperature {f} in fahrenheit is {c} in celsius.");

    println!("--------");

    // Create a mutable tupel and initialize first member with 20.0 and second with 0.0
    let mut temp = (20.0, 0.0);
    // Print both values from tupel
    println!("Tupel values are: {0} and {1}", temp.0, temp.1);
    // Set second member of tuple to return value from function with first member as argument
    temp.1 = fahrenheit_to_celsius(temp.0);
    // Print both values from tupel
    println!("The temperature {0} in fahrenheit is {1} in celsius.", temp.0, temp.1);
}

// Create function that takes one 64 bit float as parameter and returns 64 bit float
fn fahrenheit_to_celsius(f :f64) -> f64 {
    // Convert fahreheit to celsius
    // Use expression as return statement, no semicolon needed
    (f - 32.0) * 5.0 / 9.0
}