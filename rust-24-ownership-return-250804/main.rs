
fn main() {
    // Allocate memory on the heap and store string literal "Hello" in it
    // Store the pointer, length and capacity in immutable variable
    // my_string now has ownership over the pointer (and len, cap) struct
    let my_string = String::from("Hello");

    // Use println!() macro to print value of the string
    println!("String from main is: {my_string}");

    // Create a new variable and store pointer (len, cap) from string literal in it
    // The function returns the type passed into it
    // In this case the ownership is transfered, and my_string is no longer valid
    let my_new_string = give_ownership(my_string);

    println!("String from main again is: {my_new_string}");
}

// Create function that takes type String as input and returns type String
fn give_ownership(input_string: String) -> String {
    // When called input_string takes ownership of what is passed into it
    println!("String from function is: {input_string}");
    // Here it returns ownership to the outside
    input_string
}
