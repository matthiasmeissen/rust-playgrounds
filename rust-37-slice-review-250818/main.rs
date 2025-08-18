
fn main() {
    // Allocate memeory on the heap and store string literal in it
    // Store pointer, lengtht and capacity in variable
    // This variable has ownership of the String
    let s = String::from("Hello people");
    // The function takes a reference to the String s is pointing to
    // It returns a slice of that String
    let first = first_word(&s);
    println!("The first word of {s} is {first}");


    // Sum from i32 array

    // Change String by mutable reference in function
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
        if &item == b' ' {
            // Return a slice from the beginning of the input
            // To the index of the space
            &s[0..i]
        }
    }

    // Otherwise return the full length of the input
    &s[..]
}
