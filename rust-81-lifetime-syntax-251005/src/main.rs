fn main() {
    valid_lifetimes();
}

// The 'a creates a new generic lifetime (It can have any name, but a is short and convenient)
// Generic means, that its value is flexible depending on which parameters get passed in
// We assigned the lifetime to both parameters and the return value
// Since the return value depends on the inputs, the parameters are what specify the lifetime
// By tying them all to one liftime we say that the return value has maximum the shortest of both parameter lifetimes
// This lets the borrow checker decide if the usage is valid
fn longest<'a> (a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn valid_lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "abc".to_string();

    // The result function can be used properly, since all elements live long enough
    let result = longest(&string1, &string2);
    println!("The longer string is: {}", result);
}

fn invalid_lifetimes() {
    let string1 = String::from("abcd");

    let result;

    {
        let string2 = "abc".to_string();
        // In this case the string2 (which can be a potential return value of teh function)
        // Does only exist within this scope
        result = longest(&string1, &string2);
    }

    // So when we try to use the result here it is not valid
    //println!("The longer string is: {}", result);
}