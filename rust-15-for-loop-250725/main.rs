fn main() {
    // Declare a variable and initialize with and array of numbers
    let nums = [2, 4, 8, 16, 32];
    // Use for loop to iterate through the array
    for num in nums {
        // Print the item at index of array
        println!("{num}");
    }

    println!("--------");

    // You can add the enumerate method to an array 
    // This changes an iterator that creates a value of a into creating a tupel of (index, a)
    // It only seems to work if you add the iter method before the array 
    // This is because enumerate expects an iterator to work properly
    for (index, num) in nums.iter().enumerate() {
        // Then you can both print the index and the value
        println!("The number at index {index} is {num}");
    }

    println!("--------");

    // Use the range operator to generate numbers in sequence from first to second argument (excluded)
    for num in 1..4 {
        println!("{num}");
    }

    println!("--------");

    // You can use the rev method to reverse the order of the range
    for num in (1..4).rev() {
        println!("{num}");
    }
}