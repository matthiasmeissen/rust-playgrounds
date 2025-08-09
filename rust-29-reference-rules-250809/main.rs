
fn main() {
    // Allocate memory on the heap and store the string literal "Hello" in it
    // Store the pointer, length and capacity in a variable
    let mut my_string = String::from("Hello");

    // Create new scope inside main
    {
        // Create a mutable reference to the string
        let string_ref = &mut my_string;
        println!("Mutable reference to string inside scope is: {string_ref}")
        // Mutable reference allows us to modify the value the reference is pointing to
        string_ref.push_str(" people");
    }

}
