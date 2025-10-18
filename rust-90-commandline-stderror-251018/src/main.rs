use minigrep::*;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::create(&args).unwrap_or_else(|err| {
        eprintln!("Error creating the config: {err}");
        process::exit(1);
    });

    let run_result = run(config);

    match run_result {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Application error {e}");
            process::exit(1);
        } 
    }
}