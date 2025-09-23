use std::{fs, io::Read};

fn main() {
    let username = get_username_from_file("src/hello.txt").expect("Error reading username");
    ask_print(&username);
}

fn get_username_from_file(path: &str) -> Result<String, std::io::Error> {
    let mut username = String::new();
    fs::File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}

fn ask_print(s: &String) {
    println!("Press u to get username. Presse q to exit.");
    let mut input_buffer = String::new();
    let input_result = std::io::stdin().read_line(&mut input_buffer);
    match input_result {
        Ok(_) => (),
        Err(err) => panic!("Something went wrong: {}", err),
    }

    match input_buffer.as_str().trim() {
        "u" => println!("The username is: {s}"),
        "q" => println!("Exit program"),
        _ => panic!("Unknown Character"),
    }
}