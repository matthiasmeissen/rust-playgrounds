use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut username = String::new();
    std::fs::File::open("src/hello.txt")?.read_to_string(&mut username)?;
    println!("The username is {username}");
    Ok(())
}