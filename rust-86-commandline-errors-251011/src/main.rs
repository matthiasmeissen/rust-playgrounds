
use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: Vec<String>) -> Self {
        Self { 
            query: args[1].clone(), 
            filename: args[2].clone(), 
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args);
    run(config);

}

fn run(config: Config) {
    println!("Searching for: {}", config.query);
    println!("In file {}", config.filename);

    let content = fs::read_to_string(config.filename).expect("Error reading the file.");
}
