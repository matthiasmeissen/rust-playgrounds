// The & symbol represents a reference
// It allows you to refer to some value without taking ownership of it

fn main() {
    // Allocate memory on the heap and store string literal "Hello" in it
    // Store pointer, length and capacity in immutable variable
    let my_string = String::from("Hello");
    println!("The variable is: {my_string}");

    // Call the calculate_length() function and pass reference to my_string as argument
    let len = calculate_length(&my_string);
    // You can use my_string since it still has owndership
    println!("The length of {my_string} is {len}");
}

// Takes reference to a String as input and returns its size
// After input gets out of scope, the value it refers to is not dropped 
// Since it does not have ownership over it
fn calculate_length(input: &String) -> usize {
    // The action of creating a reference is also called borrowing
    // Since input is an immutable reference (&) we can not modify it
    input.len()
}