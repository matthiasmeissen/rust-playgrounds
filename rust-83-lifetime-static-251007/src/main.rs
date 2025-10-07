use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "abc".to_string();

    // The 'static lifetime annotation is special an guarantees that it will live for the entire duration of the program
    let string3: &'static str = "This has a static lifetime";

    let result = longest(&string1, &string2, "Details");
    println!("The longer string is: {}", result);

    println!("Static str: {}", string3);
}

// Define lifetime 'a and assign to input and output of function
// It makes sure that all elements with that lifetime attatched live at max as long as the shortest of all
// The generic parameter T is connected to the trait Display
// This means it can accept any type that can be formatted
fn longest<'a, T>(x: &'a str, y: &'a str, annotation: T) -> &'a str 
where 
    T: Display 
{
    let return_value = if x.len() > y.len() { x } else { y };
    println!("Result with {} is: {}", annotation, return_value);
    return_value
}