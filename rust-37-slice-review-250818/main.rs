fn main() {
    // Allocate memeory on the heap and store string literal in it
    // Store pointer, lengtht and capacity in variable
    // This variable has ownership of the String
    let mut s = String::from("Hello people");
    // The function takes a reference to the String s is pointing to
    // It returns a slice of that String
    // The formal way to pass the whole tring as slice to the function woul be &s[..]
    // But the compiler automatically converts &s to that
    let first = first_word(&s);
    println!("The first word of {s} is {first}");

    // Create integer array
    let nums = [4, 8, 32, 80, 120];
    // Pass a slice from that array into function and store return value in variable
    let sum = get_sum(&nums[0..3]);
    println!("The sum of the first three items of the array is: {sum}");

    // Create a mutable reference to s and add a string literal (is type &str) as input
    // The mutale reference only exists withing the function
    append_to_string(&mut s, " on this planet");

    // You can create another mutable reference
    let new_s = &mut s;
    new_s.push_str("and universe.");

    // You can use the original value of s in a function
    // Since the NLL (Non Lexical Lifetime) of new_s is telling the compiler it wont be used again, so it can be dropped
    println!("The value of s is: {s}");
    // But only if you do not use the new_s again afterwards
    // Since this would violate the mutablity rules
    // println!("The value of s is: {new_s}");
}

// This function takes a reference of type String slice as parameter
// And returns a reference to a String slice
fn first_word(s: &str) -> &str {
    // Create an array of each item from input slice as byte type
    let bytes = s.as_bytes();

    // Loop through the bytes array with for loop
    // Use iter() method to go through the whole array
    // And enumerate() methode to convert each item into a tupel
    // Containing a reference to the item itself and the index
    for (i, &item) in bytes.iter().enumerate() {
        // Check if the current item is a space
        if item == b' ' {
            // Return a slice from the beginning of the input
            // To the index of the space
            return &s[0..i];
        }
    }

    // Otherwise return the full length of the input
    &s[..]
}

// This function accepts a reference to a 32bit integer slice and returns its sum
// For comparison a referecne to an 32bit integer array would look like this &[i32; 5]
// The crucial distinction is that an array needs a size as well
fn get_sum(i: &[i32]) -> i32 {
    // Use the iter() method to iterate through array
    // And the sum() method to add all values together
    i.iter().sum()
}

// This function accepts a mutable reference to a String
// And a string slice as input
fn append_to_string(s: &mut String, val: &str) {
    println!("Will add: {val} to: {s}");
    // It will add the string slice to the String
    s.push_str(val);
    println!("Input is now: {s}");
}