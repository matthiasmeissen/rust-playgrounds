
use std::fs;

fn main() {
    let file_read_result = fs::read("src/hello.txt");
    match file_read_result {
        Ok(file) => println!("File read: {:?}", file),
        Err(err) => panic!("Error reading the file: {}", err),
    };
}
