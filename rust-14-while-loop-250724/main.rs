fn main() {
    // Declare mutable variable and initialize with 0
    let mut x = 0;
    // Create a while loop that runs its content as long as condition is true
    while x < 5 {
        // Increment x by 1
        x += 1;
        // Print x with println!() macro
        println!("{x}")
    }
    // Print line with done
    println!("Done");

    // Create an array with numbers
    let numbers = [2, 4, 8, 16, 32];
    // Declare a mutable variabel for index and initialize to 0
    let mut index = 0;
    // Create a whiel loop that runs when the condition is true
    while index < 5 {
        // Print the number at current index in the array
        println!("The {0} number in the array is {1}", index, numbers[index]);
        // Add one to the index
        index += 1;
    }
}