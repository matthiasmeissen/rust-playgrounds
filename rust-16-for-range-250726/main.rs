fn main() {
    // Iterate through rang 1-5 (last number is excluded)
    for num in 1..6 {
        // Print current index
        println!("{num}");
    }

    println!("--------");

    // Iterate through numbers 1-5 but reversed using the rev methon on the range type
    for num in (1..6).rev() {
        println!("{num}");
    }

    println!("--------");

    // Declare a variable (immutable) and initialize with array of numbers
    let a = [2, 4, 8, 16, 32];
    // Use a for loop to iterate through the array
    // Convert variable to iterable tyle to then use rev method
    // The range type (1..4) is already iteratble so you do not need to use iter method on it
    for num in a.iter().rev() {
        println!("{num}");
    }

    println!("--------");

    let num = 8;
    let val = get_fibonacci(num);
    println!("The {num} fibonacci number is: {val}");
}

// Create function with one argument (32bit unsigned integer) and one return value of same type
fn get_fibonacci(x: u32) -> u32 {
    // Declare mutable variable to hold current number, initialize to 1 (first number in sequence)
    let mut current = 1;
    // Declare mutable variabel to hold previous number, initialize to 0
    let mut prev = 0;
    // Create a loop that repeats x amount of times
    for _num in 1..x {
        let temp = current;
        // Set current number (fibonnaci number at index) to prev + current
        current = prev + current;
        // Set prev to current
        prev = temp;
        println!("Debug: {current}");
    }
    // Return current number
    current
}