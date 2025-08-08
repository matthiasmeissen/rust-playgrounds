fn main() {
    // Allocate memeory on the heap and store the string literal "Hello" in it
    // Then store pointer, length and capacity in a mutable variable
    // It is important to make the variable mutable, otherwise borrowing wont work
    let mut my_string = String::from("Hello");
    println!("The string is: {my_string}");

    // Call change function and pass mutable reference to String as argument
    change(&mut my_string);
    println!("The string is now: {my_string}");

    // Create new mutable reference to String in main
    let my_ref_string = &mut my_string;
    println!("The new reference is: {my_ref_string}");

    // Doing this again is still valid, since we have used my_ref_string in the println!() macro
    let my_ref_string2 = &mut my_string;
    println!("The new reference is: {my_ref_string2}");
    
    // The following is not valid, since we can only have one mutable reference at a time
    //let ref1 = &mut my_string;
    //let ref2 = &mut my_string;
    //println!("ref1 is: {ref1} and ref2 is: {ref2}");

    // This only applies to the current scope and is intended to prevent data races
    // When we use it in a seperate scope it is valid again
    {
        let ref1 = &mut my_string;
        println!("The string ref1 is pointing to is: {ref1}");
    }
    let ref2 = &mut my_string;
    println!("The string ref2 is pointing to is: {ref2}");
}

// Create a function that has a mutable reference to type String as a parameter
fn change(input: &mut String) {
    // Append a string literal to the referenced String
    input.push_str(" people");
}