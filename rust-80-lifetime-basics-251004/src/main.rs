fn main() {
    let string1 = String::from("abcd");
    let string2 = "abc";

    let result = longest(&string1, &string2);
    println!("The longer string is: {}", result);
}

// This wont compile, since we do not know what the function will return
// From a lifetime perspective this makes it hard for the borrow checker
// To say how long a value is valid

//fn longes(a: &str, b: &str) -> &str {
//    if a.len() > b.len() { a } else { b }
//}

// This specifies, that the returned reference will be valid
// As long as both the parameters are valid
// The <'a> defines a lifetime and here us assigned to a, b and the return value
// This means, that the returned value is tied to the lifetimes of the input parameters
// It will not live longer than the shortest of them
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}