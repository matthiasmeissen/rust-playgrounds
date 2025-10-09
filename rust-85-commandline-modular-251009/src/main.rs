use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let filename = args[2].clone();
        Self { query, filename }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    run(config);
}

fn run(config: Config) {
    let content = fs::read_to_string(&config.filename).expect("Error reading the file.");
    println!("The file {} has the content: \n{}", config.filename, content);
}