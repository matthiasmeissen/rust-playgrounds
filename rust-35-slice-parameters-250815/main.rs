
fn main() {
    let s = String::from("Hello people");
    prinln!("The value of s is : {s}");

    let word = first_word(s);
    println!("The first characters from s are: {word}");
}

// The function accepts a reference to a String as input
// And returns a reference to a string literal
fn first_word(s: &String) -> &str {
    let l = s[0..4];
    l
}
