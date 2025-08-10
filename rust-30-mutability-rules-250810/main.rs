
fn main() {
    // Allocate memory on the heap and store string literal "Hello" in it
    // Store a Struct that includes pointer, length and capacity in a mutable variable on the stack
    let mut my_string = String::from("Hello");
    // Use println!() macro to print teh value stored in heap
    println!("The value of string is: {my_string}");
    // Use push_str() method to append " people" to the string
    my_string.push_str(" people");

    let ref1 = &my_string;
    let ref2 = &my_string;
    println!("The references to String are: \n    ref1: {ref1} \n    ref2: {ref2}");

    // The compiler detected the ref1 and ref2 have been used and will not be used anymore
    // This is called a NLL (Non Lexical Lifetimes) 
    // and means that we can now create a mutable reference to the String
    let ref3 = &mut my_string;
    ref3.push_str(" on this planet");
    println!("The string is now: {ref3}");

    // Create a new immutable reference to the String
    let ref4 = &my_string;
    // The following line is valid, since ref3 has already been used
    println!("The string is now: {ref4}");
    // This line is not valid, since it violates the rule that 
    // you can have only one mutable referrence or any number of immutable references to a value at a time
    //println!("The values are now: \n ref3:    {ref3} \n ref4:    {ref4}");
}
