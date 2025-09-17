use std::io;

fn main() {
    println!("Lets crash a program.");

    loop {
        println!("Type 1 for panic, and 2 for array out of bounds.");

        let choice = match get_input() {
            Some(input) => input,
            None => continue
        };

        match choice.to_string().trim() {
            "1" => {
                panic!("Crash and burn");
            },
            "2" => {
                let nums = vec![0, 1, 2];
                println!("This will crash: {}", nums[8]);
                break;
            },
            _ => {continue;}
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer) {
        Ok(..) => return Some(buffer),
        Err(..) => return None,
    }
}