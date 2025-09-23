
use std::{fs, io::Read};

fn main() {
    let username = get_username_from_file("src/hello.txt").expect("Error reading username");
}

fn get_username_from_file(path: &str) -> Result<String, std::io::Error> {
    let mut username = String::new();
    fs::File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}

fn ask_print(s: &String) {
    let mut input_buffer = String::new();
    let input_result = std::io::stdin().read_line(&mut input_buffer);
    match input_result {
        Ok(_) => println!("Processing input of {}", input_buffer),
        Err(err) => panic!("Something went wrong: {}", err),
    }
}
