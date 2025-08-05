fn main() {
    // Use give ownership function to cretae String type
    // And store ownserhip with pointer, length and capacity in mutable variable
    let mut my_string = give_ownership();
    println!("String in main is: {my_string}");

    // This function takes ownership and then returns it when done
    my_string = take_and_return_ownership(my_string);
    println!("String in main is: {my_string}");

    // This function takes ownership from my_string 
    // But returns it in form of a tupel that also includes its length
    let (my_new_string, len) = calculate_length(my_string);
    println!("String of value {my_new_string} has length: {len}");

    // Since len is of type usize it is stored on teh stack and implements the copy trait
    let len2 = len;
    println!("len is: {len} and len2 is {len2}");
}

fn give_ownership() -> String {
    let generated_string = String::from("Hello");
    generated_string
}

fn take_and_return_ownership(input: String) -> String {
    println!("String inside function is: {input}");
    input
}

// To do something with a value and also give its ownership back we need to return a tupel type
fn calculate_length(input: String) -> (String, usize) {
    let len = input.len();
    return (input, len);
}