use std::env;
use std::fs;

fn main() {
    // The env::arg returns an iterator of argments passed in by the command line
    // The .collect() method turns it into a collection (here we need to annotate the type)
    let args: Vec<String> = env::args().collect();
    // The dbg!() macro prints the arguments to the console
    dbg!(&args);

    let query = &args[1];
    let file_path = &args[2];

    let content = fs::read_to_string(file_path).expect("This file does not exist.");

    println!("Searching for {query}");
    println!("In file {file_path}");
    println!("With the content: \n{content}");
}