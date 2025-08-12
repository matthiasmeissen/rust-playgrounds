// You can have either one mutable reference or any number of immutable reference to a variable at a time
// The compiler detect last usage of values and drops them using NLL (Non Lexical Lifetimes)
// This lets you use it in the same scope again

fn main() {
    // Allocate memory on the heap and store string literal "Hello" in it
    // Store the Struct including pointer, length and capacity in mutable variable on the stack
    // This variable has ownership now
    let mut my_string = String::from("Hello");
    println!("The value of String is: {my_string}");

    {
        // Inside this new scope we can access the original reference
        // We can create as many immutable references as we want
        let new_string = &my_string;
        let new_string2 = &my_string;
        let new_string3 = &my_string;
        println!("-------- \nNew String 1: {new_string} \nNew String 2: {new_string2} \nNew String 3: {new_string3} \n--------");

        // When all immutable references are used and wont be used again, we can create a mutable reference
        let new_string4 = &mut my_string;
        new_string4.push_str(" to all");
        println!("The value of String inside the scope is: {new_string4}");
    }

    // This variable borrows mutable reference (is allowed to manipulate)
    let new_string = &mut my_string;
    new_string.push_str(" people");
    // The new_string reference is called the last time here
    // This means the compiler can detect that we no longer need it and drop it 
    // This is called NLL (Non Lexical Lifetime) 
    println!("The value of String is now: {new_string}");

    // Now we create a second mutable reference to the String
    // This is only valid when we no longer access the new_string reference in the following code
    let new_string2 = &mut my_string;
    new_string2.push_str(" on this planet");
    println!("The value of String is now: {new_string2}");
}