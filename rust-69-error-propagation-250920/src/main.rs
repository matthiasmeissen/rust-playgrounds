
use std::{fs::File, io::Read};

fn main() {

    // Call File::open() function which tries to open a Filestream
    let file_open_result = File::open("src/hello.txt");

    // Desctructure the Result and on success store the Filestream Handle in variable 
    let mut username_file = match file_open_result {
        Ok(file) => file,
        Err(err) => panic!("Error opnening the file: {:?}", err),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    };

}
