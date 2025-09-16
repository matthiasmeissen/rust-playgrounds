use std::io;

mod database;
use database::*;

fn main() {
    let mut database = Database::new();

    loop {
        println!("What do you want to do: \n1: Add Employee \n2: Get data \n3: Exit");

        let choice = match get_user_input("Enter your choice") {
            Some(input) => input,
            None => continue
        };

        match choice.as_str() {
            "1" => {
                let name = match get_user_input("Enter employee name") {
                    Some(name) => name,
                    None => continue
                };
                let department = match get_user_input("Enter department") {
                    Some(department) => department,
                    None => continue
                };
                database.add_data(name, department);
            },
            "2" => {
                database.print_data();
            },
            "3" => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Please enter either 1, 2 or 3.");
            }
        }

    }

}

fn get_user_input(inst: &str) -> Option<String>{
    println!("{inst}");
    let mut buffer = String::new();

    if io::stdin().read_line(&mut buffer).is_ok() {
        return Some(buffer.trim().to_string());
    }
    None
}