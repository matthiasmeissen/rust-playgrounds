use std::fs;
use std::io::{Error, Read};

fn main() {
    let username = get_username_from_file("src/hello.txt").expect("Something went wrong.");
    println!("The username is: {username}");

    // To read a file to string is so common that there is a function for it
    let username = fs::read_to_string("src/hello.txt").expect("Something went wrong.");
    println!("The username is: {username}");
}

// Create a function that returns a Result which is a String on success and an io::Error on fail
fn get_username_from_file(path: &str) -> Result<String, Error> {
    // Create new String to store the username
    let mut username_buffer = String::new();
    // Try to open a file to a specific path
    // On success take that result and try to read to string
    // On success store String in username_buffer variable
    // When either of them fails, return the Err() as a Result
    fs::File::open(path)?.read_to_string(&mut username_buffer)?;
    // On success return username_buffer as Result
    Ok(username_buffer)
}