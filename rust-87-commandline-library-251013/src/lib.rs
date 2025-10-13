use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    filepath: String,
}

impl Config {
    pub fn create(input: &[String]) -> Result<Self, &'static str> {
        if input.len() < 3 {
            return Err("Not enough parameters");
        }

        let query = input[1].clone();
        let filepath = input[2].clone();

        Ok(Self { query, filepath })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filepath)?;
    println!("The content of the file {} is: \n{}", config.filepath, content);
    Ok(())
}