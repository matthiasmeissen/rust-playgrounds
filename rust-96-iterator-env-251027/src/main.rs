#[derive (Debug)]
struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn build(mut args: std::env::Args) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(val) => val,
            None => return Err("Please provide a query")
        };

        let filepath = match args.next() {
            Some(val) => val,
            None => return Err("Please provide a filepath")
        };

        Ok(Self { query, filepath })
    }
}

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Something went wrong: {err}");
        std::process::exit(1);
    });

    println!("The config is: {:?}", config);
}