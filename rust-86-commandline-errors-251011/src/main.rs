mod lib;
use crate::lib::search;

use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Please add more arguments");
        } 

        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Self { query, filepath})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problme parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file {}", config.filepath);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}