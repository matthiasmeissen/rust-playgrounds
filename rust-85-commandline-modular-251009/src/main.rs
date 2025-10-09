
use std::env;

fn main() {
    let args = env::args();

    for argument in args {
        println!("{argument}");
    };
}
