// When you allocate some memory on the heap
// This memory is only available in the current scope
// Once we get out of it, the memory will be dropped

fn main() {
    // Create mutable variable and initialize it with a string literal
    // Its text content has a fixed size and is hardcoded directly into the programs binary file
    // The variable my_string_literal is a pointer to that
    let mut my_string_literal = "String";
    println!("{my_string_literal}");

    // Since the variable is mutable you can change it to point somewhere else
    // In this case another string literal that is also hardcoded in the programs binary file
    my_string_literal = "More String";
    println!("{my_string_literal}");

    // Create a mutable variable and initialize it with the String type
    // The String type copys the data from the string literal into a new block of memory allocated on the heap
    // However the variable my_string us a smart pointer
    // It contains a pointer to the memory, the length of the string, the capacity (how much memory is allocated on the hep for it)
    let mut my_string = String::from("Hello people");
    println!("{my_string}");

    // The push_str() method is only available for the String type
    // It appends a string to the end of oure heap allocated data
    // When there is not enough space 
    // The memory allocator will automatically allocate a large block of memory and copy the old data there
    my_string.push_str(" of this planet.");
    println!("{my_string}");
}