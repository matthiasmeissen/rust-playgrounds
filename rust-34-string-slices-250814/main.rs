fn main() {
    let s = String::from("Hello people");
    println!("The value of s is: {s}");

    // You can use a string slice 
    // to reference a continuos sequence of characters in a string

    // A slice has the following syntax
    // [starting_index .. ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last postion

    // A slice contains a pointer and a length
    // The slice called first contains a pointer to the index 0
    // And a length of 5

    let first = &s[0..5];
    let second = &s[6..s.len()];

    println!("First word of {s} is: {first} \nSecond word of {s} is: {second}");

    // There are different way to specify the first and last indices of the slice
    // When the starting index is 0 you do not need to add it
    // When the ending index is the length of the string you also do not need it
    let first1 = &s[..5];
    let second1 = &s[6..];

    println!("-------- \nFirst word of {s} is: {first1} \nSecond word of {s} is: {second1}");
}