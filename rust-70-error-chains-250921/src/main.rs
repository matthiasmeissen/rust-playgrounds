
use std::fs;
use std::io;
use std::io::Read;

fn main() {
    let username = get_username_from_file().expect("Something went wrong.");
}

// Create a function that returns a Result which is a String on success and an io::Error on fail
fn get_username_from_file() -> Result<String, io::Error> {
    // Create new String to store the username
    let mut username_buffer = String::new();
    // Try to open a file to a specific path
    // On success take that result and try to read to string
    // On success store String in username_buffer variable
    // When either of them fails, return the Err() as a Result
    fs::File::open("src/hello.txt")?.read_to_string(&mut username_buffer)?;
    // On success return username_buffer as Result
    Ok(username_buffer)
}
