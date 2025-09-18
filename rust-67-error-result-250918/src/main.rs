use std::{fs::File, io::Read};

fn main() {
    try_open_file("src/hello.txt");

    try_open_file("test.txt");
}

fn try_open_file(path: &str) {
    let file_handel = File::open(path);
    let mut result =  match file_handel {
        Ok(file) => {
            println!("File read success.");
            file
        },
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut buffer = String::new();
    let file_content = result.read_to_string(&mut buffer);

    println!("The file is: {}", &buffer.as_str().trim());
}