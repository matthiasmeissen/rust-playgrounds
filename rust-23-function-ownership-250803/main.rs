
fn main() {
    // Allocate memory on the heap and store the string literal "Hello" in it
    // Assing the pointer to that address (along with length and capacity) to a variable and store it on the stack
    let my_string = String::from("Hello");

    println!("String from main: {my_string}");

    // Call take_ownership() function and pass my_string as an argument
    take_ownership(my_string);

    println!("String from main again: {my_string}");
}

// Create a function with a parameter of type String, that prints it to the console
fn take_ownership(some_string: String) {
    println!("String from function: {some_string}");
}
