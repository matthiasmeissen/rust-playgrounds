use std::io::{stdin, Read};
use std::fs::File;

fn main() {
    let input_filename = &get_user_input();

    let path = input_filename.as_str().trim();

    match read_file(path) {
        Ok(s) => println!("{path} has the following content: \n{s}"),
        Err(err) => println!("{path} does not exist, please also add filetype. {err}"),
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut buffer = String::new();

    let path_formatted = format!("src/{path}");

    File::open(path_formatted)?.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn get_user_input() -> String {
    println!("Enter a filename to print its context:");
    // Create mutable buffer to store user input
    let mut input_buffer = String::new();

    loop {
        // Try to read line input
        stdin().read_line(&mut input_buffer).expect("Error reading input");
    
        // Check if input is empty
        if input_buffer.as_str().trim().len() == 0 {
            println!("Can not read empty input. Please try again.");
            continue;
        } else {
            return input_buffer;
        }
    }
}