
fn main() {
    // Allocate memory on the heap and store string literal "Hello" in it
    // Store the Struct including pointer, length and capacity in mutable variable on the stack
    // This variable has ownership now
    let mut my_string = String::from("Hello");
    prinln!("The value of String is: {my_string}");

    // This variable borrows mutable reference (is allowed to manipulate)
    let new_string = &mut my_string;
    new_string.push_str(" people");
    println!("The value of String is now: {new_string}");
}
