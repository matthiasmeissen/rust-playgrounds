fn main() {
    // Allocate memory on the heap and store the string literal "Hello" in it
    // Assing the pointer to that address (along with length and capacity) to a variable and store it on the stack
    let my_string = String::from("Hello");

    println!("String from main: {my_string}");

    // Call take_ownership() function and pass my_string as an argument
    // The function has now taken ownership of the value (stores its pointer)
    take_ownership(my_string);

    // Using the my_string variable again is not valid, since it does not store the reference anymore
    //println!("String from main again: {my_string}");

    // In fact the string literal "Hello" stored on the heap is now gone completely
    // When we have passed it into the function it was dropped from main
    // And when the function was done it was dropped from there as well

    // In contrast to that the type i32 implements the Copy Trait
    // That means when it is used somewhere else we create a copy of it on the stack
    let x = 20;
    println!("Integer from main: {x}");
    make_copy(x);
    println!("Integer from main again: {x}");
}

// Create a function with a parameter of type String, that prints it to the console
fn take_ownership(some_string: String) {
    println!("String from function: {some_string}");
}

// Create a function with parameter of type 32bit integer, that prints it to the console
fn make_copy(some_integer: i32) {
    println!("Integer from function: {some_integer}");
}