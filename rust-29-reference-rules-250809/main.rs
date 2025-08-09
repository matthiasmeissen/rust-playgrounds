// Reference Rules
// You can either have one mutable reference or any number of immutable references at a time.
// Or to put differently: If you have one mutable reference to a value, you can have no other references to that value.

// Why this helps
// This mechanism prevents so called data races
// They occur, when more than on references to a value exist
// And when at least one of them is used to write on that data
// And when there is no mechanism in place the handle synchronized access to that data
// Which might result in undefined behavior which is very hard to track

fn main() {
    // Allocate memory on the heap and store the string literal "Hello" in it
    // Store the pointer, length and capacity in a variable
    let mut my_string = String::from("Hello");
    println!("The value of stinrg inside main is: {my_string}");

    // Create new scope inside main
    {
        // Create a mutable reference to the string
        let string_ref = &mut my_string;
        println!("  Mutable reference to string inside scope is: {string_ref}");
        // Mutable reference allows us to modify the value the reference is pointing to
        string_ref.push_str(" people");
        println!("  Mutable reference to string inside scope after push is: {string_ref}");

        // You can create another mutable reference to my_string
        // since the scope of one variable ends after it is used the last time
        // This is called Non Lexical Lifetimes (NLL) and is a special feature 
        // where the compiler analyzes you code and detects where the pointer was used the last time to acces the value
        let string_ref2 = &mut my_string;
        println!("  Mutable reference 2 to string inside scope is: {string_ref2}");

        // This will create an error when trying to create string_ref2
        // Since it does not respect the rule that you only can have one mutable reference at a time
        // println!("Mutable reference to string inside scope after push is: {string_ref}");

        {
            // When you create another scope, this is valid again
            let string_ref = &mut my_string;
            println!("      Mutable reference to string inside second scope is: {string_ref}");
        }
    }
}