
use std::env;
use std::process;

struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn create(input: &[String]) -> Result<Self, &'static str> {
        if input.len() < 3 {
            return Err("Not enough parameters");
        } else {
            let query = input[1].clone();
            let filepath = input[2].clone();
            return Ok(
                Self { query, filepath }
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::create(&args).unwrap_or_else(|err| {
        eprintln!("Error creating the config {err}");
        process::exit(1);
    });
}
