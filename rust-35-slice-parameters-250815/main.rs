fn main() {
    // Create String on heap
    let s = String::from("Hello people");
    println!("The value of s is : {s}");

    // Pass reference to String into first word function
    // Store return (&str a string literal) in variable
    let word = first_word(&s);

    // This will not work, since it violates reference rules
    // You can only have either one mutable reference to a variable or any number of immutable ones
    // In the first_word() function we have create an immutable reference
    // Since the clear() function is using a mutable reference this is not allowed
    //s.clear();

    println!("The first word in s is: {word}");

    // This uses an optimized way of the function
    // That takes a slice as input
    // Which makes it more flexible to use
    let word1 = first_word1(&s[..]);
    println!("The first word of s is: {word1}");

    // This is a string literal and it will be stored directly in the binary
    // So the variable sentence is of type &str
    // It is a slice pointing to a specific point in the binary
    // This is also the reason why slices are immutable
    let sentence = "Hello people on this planet";
    println!("{sentence}");

    let word2 = first_word1(sentence);
    println!("The first word of sentence is: {word2}");
}

// The function accepts a reference to a String as input
// And returns a reference to a string literal
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word1(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}