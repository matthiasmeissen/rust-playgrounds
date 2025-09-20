use std::{fs::File, io::{self, Read}};

fn main() {

    let username = read_username_from_file2().unwrap();
    println!("The complex username is: {username}");

    let username = read_username_from_file1().unwrap();
    println!("The better username is: {username}");

    let username = read_username_from_file().unwrap();
    println!("The shortest username is: {username}");

}

fn read_username_from_file2() -> Result<String, io::Error> {
    // Call File::open() function which tries to open a Filestream
    let file_open_result = File::open("src/hello.txt");

    // Desctructure the Result and on success store the Filestream Handle in variable
    // In case this is successfull we write the file to the username_file
    // Otherwise we exit the function and return the error
    let mut username_file = match file_open_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    // When the previous match statement was successfull we add a new String
    let mut username = String::new();

    // Then we try to read the file to a String
    // When this was successfull, we return an Ok(username)
    // When this was not successfull, we return the error
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn read_username_from_file1() -> Result<String, io::Error> {
    // When a function returns an Result<T,E> we use the ? operator
    // This returns the T on success
    // And propagates the E as an error
    let mut username_file = File::open("src/hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    // This is even shorter, since we can add a method directly to a function result
    // The ? means, when I am successfull take the value
    // Otherwise exit teh function and return the error message
    File::open("src/hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}