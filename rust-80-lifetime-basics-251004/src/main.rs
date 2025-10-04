
fn main() {
    let string1 = String::from("abcd");
    let string2 = "abc";

    let result = longest(&string1, &string2);
    println!("The longer string is: {}", result);
}

fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}
