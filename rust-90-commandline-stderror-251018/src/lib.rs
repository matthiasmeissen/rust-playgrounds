use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    query: String,
    filepath: String,
    ignore_case: bool,
}

impl Config {
    pub fn create(input: &[String]) -> Result<Self, &'static str> {
        if input.len() < 3 {
            return Err("Not enough parameters");
        }

        let query = input[1].clone();
        let filepath = input[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self { query, filepath, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath)?;

    let search_type;
    if config.ignore_case == true {
        search_type = search_insensitive(&config.query, &contents);
    } else {
        search_type = search(&config.query, &contents);
    }

    for line in search_type {
        println!("{line}");
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let lowercase_query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&lowercase_query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust us.";
        assert_eq!(vec!["Rust:", "Trust us."], search_insensitive(query, contents));
    }
}