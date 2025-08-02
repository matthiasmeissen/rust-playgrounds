// Data stored on the stack has a fixed size and is relatively small
// This means it will be copied when you assign it to another variable

// In contrast data stored on the heap will transfer ownership
// This is helpful since the data stored in there might be huge

// Details
// let text = String::from("Hello"); 
// Allocates memory on the heap and stores the string literal "Hello" in there
// Creates a String struct on the stack (the variable called text) 
// that holds the pointer to the heap memory, the length and the capacity

// let x = 20;
// Create a value of 20 and pushes it to the stack
// The i32 type has implemented the Copy trait (like most scalar types and tuples that only consist of sclar types as well)
// This means that it does not move, but is just copied

fn main() {
    // Declare an immutable variable and initialize it with an i32 type 
    // This is a scalar type, has a fixed size and is stored on the stack
    let x = 20;
    // Declare another immutable variable and initilaize it with another stack variable
    // This copys the value stored in memory
    let y = x;

    // There is no transfer of ownership for scalar types, which means we can use them both
    println!("x is: {x} and y is: {y}");
}