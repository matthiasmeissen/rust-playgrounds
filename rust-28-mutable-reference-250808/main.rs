
fn main() {
    // Allocate memeory on the heap and store the string literal "Hello" in it
    // Then store pointer, length and capacity in immutable variable
    let my_string = String::from("Hello");
    println!("The string is: {my_string}");

    // Call change function and pass mutable reference to String as argument
    change(&mut my_string);
    println!("The string is now: {my_string}");
}

// Create a function that has a mutable reference to type String as a parameter
fn change(input: &mut String) {
    // Append a string literal to the referenced String
    input.append(" people");
}
