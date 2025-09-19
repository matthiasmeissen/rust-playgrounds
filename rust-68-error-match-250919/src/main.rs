use std::fs;

fn main() {
    // The read_to_string() function returns a Result<T, E> enum
    // T and E arg generic and can be of any type
    // T is the element that gets returned when action was successfull
    // E is the element that gets returned on an error
    let file_read_result = fs::read_to_string("src/hello.txt");
    // The most direct way to get the values of T and E is witch a match statement
    // Here, the Ok(T) and Err(E) lets you access the elements and define what to do with them
    match file_read_result {
        Ok(file) => println!("File read: {}", file),
        Err(err) => panic!("Error reading the file: {}", err),
    };

    // A shorter way is the .expect() method, which can be called on a Result<T,E>
    // This just returns T on success and panics with a user specified error message on error
    // There is also an .unwrap() method, which expects the result to ge ok and panics when not
    let file = fs::read_to_string("src/hello.txt").expect("Error reading the file");
    // Note that the read_to_string() function returns the comlete file
    // In order remove whitespaces and linebreaks you can use the .trim() method
    println!("The file contains: {}", file.trim());
}