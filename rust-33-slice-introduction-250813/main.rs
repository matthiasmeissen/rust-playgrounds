fn main() {
    // Allocate memory on the heap and store string literal "Hello people" in it
    // Store pointer, length and capacity as struct in a mutable variable
    let mut s = String::from("Hello people");
    let word = first_word(&s);
    println!("The first word ends at character number {word}");

    // We can clear the String with the clear() method
    // The variable must be mutable for that
    s.clear();
    // Now we do not have the String anymore, but still can get the length of the first word
    // Meaning that the String and the number are not connected in any way
    // This is problematic since it can lead to errors that the compiler can not detect in advance
    println!("The not existing first word ends at character number {word}");
}

// This function returns the index of the last character in the first word of a string
fn first_word(s: &String) -> usize {
    // Convert string into an array of bytes
    // This helps to detect the empty spaces
    let bytes = s.as_bytes();

    // The iter() method returns a reference to each element in a collection (in the bytes array)
    // The enumerate() method wraps the result of iter() and returns a tupel instead
    // Where the first element is the index and the second is the element itself

    // So this loop goes through every item in the bytes array
    // Checks if an item is a space (using the byte literal syntax) and then returns its index
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    // Otherwise we return the length of the string
    s.len()
}