fn main() {
    // Create an array of integers on the stack 
    // And store the pointer (and length) to it in a variable called nums
    let nums = [4, 8, 12, 16, 48, 80];

    // Create a slice (immutable reference) to some items in the array (index 0, 1, 2)
    // Store in a pointer (and length of slice) into variable
    let num_slice = &nums[0..3];

    // Caluclate length of slice and store value in variable
    let slice_len = num_slice.len();
    println!("The length of num_slice is: {slice_len}");

    // The iteration operator lets you select a portion of an array by specifing the start index and one more the the end index
    // It is a reference to the original array
    print_length(&nums[0..4]);

    let sum = calculate_range_values(&nums[0..5]);
    println!("The sum of the values is {sum}");

    let sum = calculate_range_values(&nums);
    println!("The sum of the values is {sum}");

    let sum = get_array_sum(&nums);
    println!("The sum of the values with another function is {sum}");
}

// I am trying to create a function that takes in a slice of an integer array and prints its length
// This function takes a reference to an 32bit integer array
fn print_length(s: &[i32]) {
    // It then uses the len() methods to calculate its length and prints it
    println!("The length of the slice is: {}", s.len());
}

// This function takes a 32bit integer array as input and returns a 32bit integer
fn calculate_range_values(s: &[i32]) -> i32 {
    // Create mutable integer varaibel and initialize to zero
    let mut num = 0;
    // Loop through the input array
    for i in s {
        // Add the value at index to sum variable for each loop
        num = num + i;
    }
    // Return the num value
    num
}

// This function is way more compact as the one befor
fn get_array_sum(s: &[i32]) -> i32 {
    // Iterate through each item in the array and get the sum of all
    s.iter().sum()
}