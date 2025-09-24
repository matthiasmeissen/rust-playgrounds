
use std::io::stdin;

fn main() {
    println!("Enter a filename to print its context:");

    // Create mutable buffer to store user input
    let mut input_buffer = String::new();
    // Try to read line input
    stdin().read_line(&mut input_buffer).expect("Error reading input");

    let response = input_buffer.as_str().trim();

    if response.len() == 0 {
        panic!("Can not read empty input.");
    }
}
