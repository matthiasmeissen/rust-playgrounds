fn main() {
    // Use a function to create a string literal and store it on the heap
    // And then return the ownership to that pointer (and len, cap) struct
    let my_string = give_ownership();

    // Use println!() macro to print value of the string
    println!("String from main is: {my_string}");

    // Create a new mutable variable and store pointer (len, cap) from string literal in it
    // The function returns the type passed into it
    // In this case the ownership is transfered, and my_string is no longer valid
    let mut my_new_string = take_and_return_ownership(my_string);

    println!("String from main again is: {my_new_string}");

    // You can also also reasign ownership it back to itself by using a mutable variable
    my_new_string = take_and_return_ownership(my_new_string);
    println!("String from main again is: {my_new_string}");
}

fn give_ownership() -> String {
    // Allocate memory on the heap and store string literal "Hello" in it
    // Store the pointer, length and capacity in immutable variable
    let my_string = String::from("Hello");
    // Return that ownership
    my_string
}

// Create function that takes type String as input and returns type String
fn take_and_return_ownership(input_string: String) -> String {
    // When called input_string takes ownership of what is passed into it
    println!("String from function is: {input_string}");
    // Here it returns ownership to the outside
    input_string
}