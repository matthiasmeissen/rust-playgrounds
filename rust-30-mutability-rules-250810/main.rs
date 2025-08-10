
fn main() {
    // Allocate memory on the heap and store string literal "Hello" in it
    // Store a Struct that includes pointer, length and capacity in a mutable variable on the stack
    let mut my_string = String::from("Hello");
    // Use println!() macro to print teh value stored in heap
    println!("The value of string is: {my_string}");

    let ref1 = &my_string;
    let ref2 = &my_string;
    println!("The references to String are \n ref1: {ref1} \n ref2: {ref2}");
}
